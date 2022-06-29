from display import PrintSummoner, get_input, load_data
from op_gg_scraper import Summoner 

def main():
    '''gets input from user (defult values: region-kr, player name-hide on bush)'''
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

