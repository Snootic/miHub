import sqlite3
import os

class HoyolabDatabase:
    def __init__(self):
        self.db_name = "data/hoyolab.db" if os.path.exists("data") else "../data/hoyolab.db"
        self._database_scripts_dir = "data/database_scripts" if os.path.exists("data/database_scripts") else "../data/database_scripts"
        for table in os.listdir(self._database_scripts_dir):
            self._create_table(table) if "hoyolab" in table else None
    
    def _create_table(self, table: str):
        with open(f"{self._database_scripts_dir}/{table}", "r") as f:
            query = f.read()
            
        with sqlite3.connect(self.db_name) as conn:
            cursor = conn.cursor()
            cursor.execute(query)
            conn.commit()
            
    def insert_user(self, cookies: dict):
        if self.get_user(int(cookies['hoyolab_id'])):
            return self.update_user(cookies)
        
        cookies.setdefault('account_id_v2', None)
        cookies.setdefault('ltuid_v2', None)
        
        with sqlite3.connect(self.db_name) as conn:
            cursor = conn.cursor()
            cursor.execute('''
                INSERT INTO hoyolab_users (
                    cookie_token_v2,
                    account_mid_v2,
                    account_id_v2,
                    ltoken_v2,
                    ltmid_v2,
                    ltuid_v2,
                    hoyolab_id
                ) VALUES (
                    :cookie_token_v2,
                    :account_mid_v2,
                    :account_id_v2,
                    :ltoken_v2,
                    :ltmid_v2,
                    :ltuid_v2,
                    :hoyolab_id
                )
            ''', cookies)
            conn.commit()

    def insert_redeemed_code(self, uid: int, code: str):
        with sqlite3.connect(self.db_name) as conn:
            cursor = conn.cursor()
            try:
                cursor.execute('''
                    INSERT INTO redeemed_codes (
                        uid,
                        code
                    ) VALUES (
                        :user_id,
                        :code
                    )
                ''', {"user_id": uid, "code": code})
                conn.commit()
            except:
                pass
    
    def get_all_users(self):
        with sqlite3.connect(self.db_name) as conn:
            conn.row_factory = sqlite3.Row
            cursor = conn.cursor()
            cursor.execute('''
                SELECT * FROM hoyolab_users
            ''')
            rows = cursor.fetchall()
            return [dict(row) for row in rows]
    
    def get_user(self, hoyolab_id: int):
        with sqlite3.connect(self.db_name) as conn:
            conn.row_factory = sqlite3.Row
            cursor = conn.cursor()
            cursor.execute('''
                SELECT * FROM hoyolab_users WHERE hoyolab_id = :hoyolab_id
            ''', {"hoyolab_id": hoyolab_id})
            row = cursor.fetchone()
            return dict(row) if row else None
        
    def get_user_codes(self, uid: int):
        with sqlite3.connect(self.db_name) as conn:
            cursor = conn.cursor()
            cursor.execute('''
                SELECT * FROM redeemed_codes WHERE uid = :user_id
            ''', {"user_id": uid})
            rows = cursor.fetchall()
            codes = [row[1] for row in rows]
            return codes
        
    def delete_user(self, hoyolab_id: int):
        with sqlite3.connect(self.db_name) as conn:
            cursor = conn.cursor()
            cursor.execute('''
                DELETE FROM hoyolab_users WHERE hoyolab_id = :hoyolab_id
            ''', {"hoyolab_id": hoyolab_id})
            conn.commit()
            
    def update_user(self, cookies: dict):
        cookies.setdefault('account_id_v2', None)
        cookies.setdefault('ltuid_v2', None)
        
        with sqlite3.connect(self.db_name) as conn:
            cursor = conn.cursor()
            cursor.execute(f'''
                UPDATE hoyolab_users SET
                    cookie_token_v2 = :cookie_token_v2,
                    account_mid_v2 = :account_mid_v2,
                    account_id_v2 = :account_id_v2,
                    ltoken_v2 = :ltoken_v2,
                    ltmid_v2 = :ltmid_v2,
                    ltuid_v2 = :ltuid_v2
                WHERE hoyolab_id = :hoyolab_id
            ''', cookies)
            conn.commit()
    
    