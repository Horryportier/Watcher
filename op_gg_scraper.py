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
        self.lp: int = 0
        self.win_rate: float = 0.0
        self.most_played_champions: dict = {}
    
    def get_data(self, soup: BeautifulSoup) -> list:
       self.soup = soup
       data = soup.find("meta", property="og:description")
       return data["content"] if data else None
       
       

summoner = Summoner()

url = get_url("eune", "HorryPortier6")
page = get_page(url, headers)
print(summoner.get_data(page))
