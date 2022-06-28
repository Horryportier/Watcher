import json

from rich import print
from rich.prompt import Prompt


RANK_COLORS: dict = {

    "Iron": "#453234",
    "Bronze": "#AE6A66",
    "Silver": "#607393",
    "Gold": "#DDAB57",
    "Platinum": "#0FDC95",
    "Diamond": "#74E2FE",
    "Maseter": "#EC02C2",
    "Grandmaster": "#F21F0C",
    "Challenger": "#0057E9"
}

def load_data() -> dict:
      with open("summoner.json", "r") as j:
          sum_data = json.load(j)
      return sum_data['Summoner']

def get_input() -> tuple:
    region = Prompt.ask("chose your region: def", default="kr")
    name  = Prompt.ask("Type summoner name: def", default="hide on bush")
    return region , name

class PrintSummoner:

    def __init__(self, data) -> None:
        self.data = data

    def print_raw(self) -> None:
        '''prints raw data'''
        print(f"[bold #EC02C2]{self.summoner.raw_data}")
    
    def print_info(self) -> None:
        form_data = f'''
[bold black ] Name: {self.data['name']} [/bold black ]
[bold italic {RANK_COLORS[self.data['rank']]} ] Rank: {self.data['rank']}  {self.data['sub_rank']} [/bold italic {RANK_COLORS[self.data['rank']]} ]
[bold italic #8f0fd4 ] LP: {self.data['lp']}       [/bold italic #8f0fd4 ]
[dim italic #0fe00b ] Win Rate: {self.data['win_rate']} [/dim italic #0fe00b ]
[italic #4d4348 ] Most played champions {self.data['most_played_champions']} [/italic #4d4348 ]
        '''

        print(form_data)
