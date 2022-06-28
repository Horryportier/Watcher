from display import PrintSummoner, get_input, load_data
from op_gg_scraper import Summoner 



def main():

    region , name = get_input()   
 
    '''gets player data''' 
    summoner = Summoner()
    summoner.get_summoner(region=region, sum_name=name)
    
    '''op_gg_scraper func'''
    summoner.dump_data()
    '''print  func'''
    data =  load_data()

    print_s = PrintSummoner(data)
    print_s.print_info()


if __name__ == "__main__":
    main()

