import genshin, asyncio, typing, logging
from data.database import HoyolabDatabase
from genshin.errors import AccountLoginFail

class HoyolabAccount:
    def __init__(self):
        self.client = genshin.Client()
        
        self.logger = logging.getLogger()
        
    @typing.overload
    async def _setup_cookies(self, account: str, password: str) -> dict:...

    @typing.overload
    async def _setup_cookies(self, mobile_number: str) -> dict:...

    @typing.overload
    async def _setup_cookies(self, cookies: dict) -> dict:...
    
    @typing.overload
    async def _setup_cookies(self, hoyolab_id: int) -> dict:...

    async def _setup_cookies(self, *args) -> dict:
        if len(args) == 0:
            try:
                cookies = self.client.set_browser_cookies()
            except ValueError:
                # Assuming the user is using Windows and OperaGX is the default browser
                # it will fail 'cause this browser is not currently supported by genshin client
                # so we will try to login with edge instead
                # i'll probably open a pull request to add support for OperaGX
                self.logger.warning("Failed to login with browser cookies, trying with edge")
                cookies = self.client.set_browser_cookies("edge")
            except Exception as e:
                self.logger.error(f"Failed to login with browser cookies: {e}")
                raise e
            
            cookies = self.client.cookie_manager.__dict__["_cookies"]
        
        elif len(args) == 2 and isinstance(args[0], str) and isinstance(args[1], str):
            account, password = args
            try:
                cookies = await self.client.login_with_password(account=account, password=password)
            except AccountLoginFail as ALF:
                self.logger.error(f"Failed to login with login and password: {ALF}")
                raise ALF
            cookies = cookies.to_dict()
                    
        elif len(args) == 1:
            cookies = HoyolabDatabase().get_user(int(args[0]))
            
        elif len(args) == 1 and isinstance(args[0], dict):
            cookies = args[0]
            cookies = cookies.to_dict()
            
        elif len(args) == 1 and isinstance(args[0], str):
            mobile_number = args[0]
            try:
                cookies = await self.client.login_with_mobile_number(mobile=mobile_number)
            except AccountLoginFail as ALF:
                self.logger.error(f"Failed to login with mobile number: {ALF}")
                raise ALF
            except RuntimeError as RE:
                self.logger.error(f"Failed to login with mobile number: {RE}")
                raise RE
            cookies = cookies.to_dict()
            
        else:
            message = "Invalid arguments. Please provide either account and password, mobile number, or cookies."
            self.logger.error(message)
            raise ValueError(message)
        
        if not cookies:
            self.logger.critical("Failed to login, check your credentials or cookies")
            raise AccountLoginFail("Failed to login, check your credentials or cookies")
                
        return cookies
    
    def login(self, *args):
        if not self.client.cookie_manager.user_id:
            cookies = asyncio.run(self._setup_cookies(*args))
            self.client.set_cookies(cookies=cookies)
            
            asyncio.run(self.client.get_hoyolab_user())
            
            cookies["hoyolab_id"] = self.client.hoyolab_id
            HoyolabDatabase().insert_user(cookies)
            
            self.logger.info(f"Logged in as {self.client.hoyolab_id}")
            
        with open("last_user.txt", "w") as f:
            f.write(str(self.client.hoyolab_id))
            
        return self.client.cookie_manager
    
    async def hoyolab_account(self, hoyolab_id: int = None):
        account = await self.client.get_hoyolab_user(hoyolab_id= hoyolab_id if hoyolab_id else None)
        return account
    
    async def user_games(self, cached: bool = False):
        # getting all mihoyo games accounts the user has linked to their hoyolab account
        if cached:
            pass
            return games

        user_games = await self.client.get_game_accounts()
        
        games = [x.model_dump() for x in user_games]
        
        for i, user_game in enumerate(user_games):
            games[i]["game"] = user_game.game.value
        
        return games

    async def user_game_account(self, game: str, uid: int = None):
        # filtering the user games by game and uid
        games = await self.user_games()
        if not uid:
            game_accounts = [game_account for game_account in games if game_account["game"] == game]
            
            return game_accounts
        
        for game_account in games:
            if game_account["game"] == game and int(game_account["uid"]) == uid:
                return game_account
        
        return
    
if __name__ == "__main__":
    account = HoyolabAccount()
    account.login()