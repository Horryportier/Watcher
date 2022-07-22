from sys import exit
import requests
import json

from bs4 import BeautifulSoup
from rich import print

from headers import headers


'''OP.GG PAGE'''
URL = "https://op.gg/summoners"

def get_url(region: str, sum_name: str) -> str:
    
    return f"{URL}/{region}/{sum_name}"


def get_page(url: str, headers: dict) -> BeautifulSoup:
    '''returns page as BeautifulSoup'''
    response = requests.get(url, headers=headers)
    soup = BeautifulSoup(response.content, 'html.parser')
    return soup

class Summoner:
    def __init__(self) -> None:
        
        self.name: str = "Unknown"
        self.rank: str = "Unranked"
        self.sub_rank: str = "1"
        self.lp: str = "0"
        self.win_rate: int = ""
        self.most_played_champions: list = [None]
        self.raw_data = "None"
    
    def get_data(self, soup: BeautifulSoup) -> str:
       self.soup = soup
       data = soup.find("meta", property="og:description")
       return data["content"] if data else None
       
    def set_data(self , data: str):
        if data == "Real-time LoL Stats! Check your Summoner, Live Spectate and using powerful global League of Legends Statistics!":
            print("[bold blink red] Account dose not exist")
            exit()

        self.data = data
        
        champ_data: list = []

        split_data: list = data.split("/")
        if len(split_data) < 4:
            print(f"[bold blink red] Summoner {split_data[0]} is unrranked")
            exit()
            
        champ_data = split_data[3].split(",")

        rank_data = [w for w in  split_data[1].split(" ") if w != '']

        self.name =  split_data[0]
        self.rank = rank_data[0]

        if len(rank_data) == 3:
            self.sub_rank = rank_data[1]

        self.lp = rank_data[-1]
        self.win_rate = split_data[2]
        self.most_played_champions = champ_data       
        self.raw_data = self.data

    def dump_data(self) -> None:
        dict = {
            "Summoner":{
                "name": self.name,
                "rank": self.rank,
                "sub_rank": self.sub_rank,
                "lp": self.lp,
                "win_rate": self.win_rate,
                "most_played_champions": self.most_played_champions,
                "raw_data": self.raw_data
            }
        }

        with open("summoner.json", "w") as summoner_file:
            json.dump(dict, summoner_file, indent=6)
        


    def get_summoner(self, region: str, sum_name: str):
        self.region = region
        self.sum_name = sum_name

        url = get_url(self.region, self.sum_name)
        page = get_page(url, headers)

        data = self.get_data(page)
        self.set_data(data)

    
        
        

