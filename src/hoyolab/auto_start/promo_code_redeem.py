import sys, asyncio, argparse
from src.hoyolab.promo_code import PromoCode

def parse_args():
    parser = argparse.ArgumentParser(description="Promo Code Redeem Script")
    parser.add_argument("--game", required=True, help="The game needed")
    parser.add_argument("--account", help="Hoyolab account")
    parser.add_argument("--password", help="Hoyolab account password (requires account to be passed)")
    parser.add_argument("--hoyolab-id", type=int, help="Hoyolab ID (doesn't require account or password but will only work if already logged in with account and password once)")
    parser.add_argument("--cookies", type=dict, help="Cookies (doesn't require account or password)")
    parser.add_argument("--uid", type=int, help="UID (if provided, will redeem code for the specified UID)")
    parser.add_argument("--code", help="Code (if provided, will redeem this code only)")
    return parser.parse_args()

def main(args=None):
    if args is None:
        args = parse_args()
    
    if args.account and not args.password:
        print("Password is required when account is provided")
        sys.exit(1)
    
    promo_code_args = (args.account, args.password) if args.account else ((args.hoyolab_id,) or (args.cookies,) or ())
    
    redeem = PromoCode(args.game, *promo_code_args)
    
    try:
        args.uid = int(args.uid)
    except:
        args.uid = None
        
    try:
        args.code = str(args.code)
    except:
        args.code = None

    result = asyncio.run(redeem.redeem_promotional_code(uid=args.uid, redeem_code=args.code))

if __name__ == "__main__":
    main()