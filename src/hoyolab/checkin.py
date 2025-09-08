from .account import HoyolabAccount
from genshin.errors import GenshinException
import logging

class Checkin(HoyolabAccount):
    def __init__(self, game:str, *args):
        super().__init__()
        super().login(*args)
        
        self.client.default_game = game
        
        self.logger = logging.getLogger("mihub.hoyolab.checkin")
    
    async def claim(self):                        
        user = self.client.hoyolab_id
        
        game_name = self.client.game.name
        
        try:
            reward = await self.client.claim_daily_reward()
        except GenshinException as ge:
            self.logger.error(f"{game_name} - {user} - [{ge.retcode}] {ge.original}")
        else:
            self.logger.info(f"{game_name} - {user} - Claimed {reward.name} x {reward.amount}")
            
        return

    @property
    async def claimed(self):
        return await self.client.claimed_rewards()
    
    @property
    async def monthly_rewards(self):
        return await self.client.get_monthly_rewards()
    
    @property 
    async def reward_info(self):
        return await self.client.get_reward_info()
