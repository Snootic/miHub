import sys, asyncio, argparse
from src.hoyolab.checkin import Checkin

def parse_args():
    parser = argparse.ArgumentParser(description="Daily Check-in Script")
    parser.add_argument("--game", required=True, help="The game needed")
    parser.add_argument("--account", help="Hoyolab account")
    parser.add_argument("--password", help="Hoyolab account password (requires account to be passed)")
    parser.add_argument("--hoyolab-id", type=int, help="Hoyolab ID (doesn't require account or password but will only work if already logged in with account and password once)")
    parser.add_argument("--cookies", type=dict, help="Cookies (doesn't require account or password)")
    return parser.parse_args()

def main(args=None):
    if args is None:
        args = parse_args()

    if args.account and not args.password:
        print("Password is required when account is provided")
        sys.exit(1)
    
    checkin_args: tuple = (args.account, args.password) if args.account else ((args.hoyolab_id,) or (args.cookies,) or ())
    
    checkin = Checkin(args.game, *checkin_args)
    
    result = asyncio.run(checkin.claim())

if __name__ == "__main__":
    main()