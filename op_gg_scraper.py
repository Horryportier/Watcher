import requests
from bs4 import BeautifulSoup


'''OP.GG PAGE'''
URL = "https://op.gg/summoners"

'''headers to go around 403 error'''
headers = {
 'User-Agent': 'Mozilla/5.0 (X11; Linux x86_64; rv:101.0) Gecko/20100101 Firefox/101.0'
}


def get_url(region: str, sum_name: str) -> str:
    
    return f"{URL}/{region}/{sum_name}"


def get_page(url: str, headers: dict) -> BeautifulSoup:
    '''returns page as BeautifulSoup'''
    response = requests.get(url, headers=headers)
    soup = BeautifulSoup(response.content, 'html.parser')
    return soup

class Summoner:
    def __init__(self) -> None:
        
        self.name: str = ""
        self.rank: str = ""
        self.sub_rank: str = ""
        self.lp: str = ""
        self.win_rate: float = 0.0
        self.most_played_champions: list = []
    
    def get_data(self, soup: BeautifulSoup) -> str:
       self.soup = soup
       data = soup.find("meta", property="og:description")
       return data["content"] if data else None
       
    def set_data(self , data: str):
        self.data = data
        champ_data: list = []
        split_data: list = data.split("/")
        if len(split_data) == 4:
            champ_data = split_data[3].split(",")

        rank_data = [w for w in  split_data[1].split(" ") if w != '']

        self.name =  split_data[0]
        self.rank = rank_data[0]

        if len(rank_data) == 3:
            self.sub_rank = rank_data[1]

        self.lp = rank_data[-1]
        self.win_rate = split_data[2]
        self.most_played_champions = champ_data       

    def get_summoner(self, region: str, sum_name: str):
        self.region = region
        self.sum_name = sum_name

        url = get_url(self.region, self.sum_name)
        page = get_page(url, headers)

        data = self.get_data(page)
        self.set_data(data)
        
        

