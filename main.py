from rich import print
from rich.prompt import Prompt

from op_gg_scraper import Summoner 

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

def print_results(summoner: Summoner) -> None:
    
    print(f"[bold italic {RANK_COLORS[summoner.rank]}]{summoner.rank}")

def main():
    
    region = Prompt.ask("chose your region: ", default="kr")
    name  = Prompt.ask("Type summoner name: ", default="hide on bush")

    summoner = Summoner()
    summoner.get_summoner(region=region, sum_name=name)

    print_results(summoner)


if __name__ == "__main__":
    main()

