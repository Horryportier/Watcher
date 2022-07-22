from sys import argv, exit

from display import PrintSummoner, get_input, load_data
from op_gg_scraper import Summoner 


def main():
    args = argv 
    args_len = len(args) - 1
    if args_len == 2:
        region = argv[1]
        name = argv[2]
    elif len(argv) > 2:
        print(f'watcher takes 2 argumients not {args_len}')
        exit()
    else: 
        region , name = get_input() 

    '''gets player data''' 
    summoner = Summoner()
    summoner.get_summoner(region=region, sum_name=name)

    '''op_gg_scraper.py func'''
    summoner.dump_data()
    '''display.py  func'''
    data =  load_data()

    print_s = PrintSummoner(data)
    print_s.print_info()
    
if __name__ == "__main__":
    main()
