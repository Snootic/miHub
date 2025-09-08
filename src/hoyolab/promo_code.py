
import logging
from requests import get
from .account import HoyolabAccount
from bs4 import BeautifulSoup, element
from time import sleep
from genshin.errors import RedemptionClaimed, RedemptionCooldown, RedemptionException
from data.database import HoyolabDatabase
from datetime import datetime

class PromoCode(HoyolabAccount):
    def __init__(self, game: str, *args):
        super().__init__()
        
        super().login(*args)
        
        self.client.default_game = game
        
        self.logger = logging.getLogger("mihub.hoyolab.promo_code")
    
    @staticmethod
    def get_promo_code(game: str):
        
        headers = {"User-Agent": "Mozilla/5.0 (Windows NT 10.0; Win64; x64)"}
                
        def parse_codes_wiki(source):
            soup = BeautifulSoup(source.text, 'html.parser')
            codes = []
            codes_table = soup.find('table')
            for row in codes_table.find_all('tr'):
                columns = row.find_all('td')
                if len(columns) < 4:
                    continue
                
                if "Expired" in str(columns[3]):
                    continue
                
                # code element is from the fandom wiki, span is from the ign wiki
                code_element = columns[0].find_all('code') if columns[0].find_all('code') else columns[0].find_all("span")[1]
                
                if type(code_element) == element.ResultSet:
                    codes += [code.text.strip() for code in code_element]
                    continue
                else:
                    code = code_element.text.strip()
                            
                if not code:
                    continue
                
                codes.append(code)
            return codes
                
        match game:
            case "GENSHIN":
                source = get("https://genshin-impact.fandom.com/wiki/Promotional_Code")
                return parse_codes_wiki(source)
            case "STARRAIL":
                source = get("https://honkai-star-rail.fandom.com/wiki/Redemption_Code")
                return parse_codes_wiki(source)
            case "ZZZ":
                source = get(f"https://zenless-zone-zero.fandom.com/wiki/Redemption_Code")
                return parse_codes_wiki(source)
            case _:
                return

    async def redeem_promotional_code(self, game: str|None = None, uid: int|None = None, redeem_code:str|None = None):
        if game:
            self.client.game = game
            
        user_accounts = await self.user_game_account(game=self.client.game, uid=uid)
        self.logger.debug(f"Found {user_accounts} accounts for {self.client.hoyolab_id} in {self.client.game}")
        
        game = self.client.game.name
        
        if uid:
            return await self._claim(game, user_accounts, redeem_code)
        
        for user_account in user_accounts:
            await self._claim(game, user_account, redeem_code)
            
        return
    
    async def _claim(self,game:str, user: dict, redeem_code: str|None = None):
        async def inner(redeem_code: str):
            if code in redeemed_codes:
                self.logger.warning(f"{game} - {user['server']} - {user['uid']} - Already claimed this code: {redeem_code}")
                return
            try:
                await self.client.redeem_code(code=redeem_code, uid=int(user["uid"]))
                self.logger.info(f"{game} - {user['server']} - {user['uid']} - Redeemed: {redeem_code}")
                HoyolabDatabase().insert_redeemed_code(user['uid'], redeem_code)
                self.logger.debug(f"Inserted {redeem_code} into database for user {user['uid']} on server {user['server']}")
            
            except RedemptionCooldown:
                sleep(5)
                await inner(redeem_code)
                
            except RedemptionClaimed as re:
                self.logger.warning(f"{game} - {user['server']} - {user['uid']} - [{re.retcode}] {re.original}: {redeem_code}")
                HoyolabDatabase().insert_redeemed_code(user['uid'], redeem_code)
                self.logger.debug(f"Inserted {redeem_code} into database for user {user['uid']} on server {user['server']}")
            
            except RedemptionException as re:
                self.logger.error(f"{game} - {user['server']} - {user['uid']} - [{re.retcode}] {re.original}: {redeem_code}")
                
        redeemed_codes = self.claimed(user['uid'])
        
        if redeem_code:
            await inner(redeem_code)
            
        else:
            codes = self.get_promo_code(game)
            if not codes:
                self.logger.error(f"Failed to get codes for {game}")
                return
            self.logger.debug(f"Found {len(codes)} codes for {game}")
            for code in codes:
                await inner(code)
                    
        return

    def claimed(self, uid: int):
        codes = HoyolabDatabase().get_user_codes(uid)
        logging.debug(f"found codes: {codes} for uid: {uid}")
        return codes