import requests
from bs4 import BeautifulSoup 

url = 'https://eune.op.gg/summoners/eune/I am Sobek'

headers = {
 'User-Agent': 'Mozilla/5.0 (X11; Linux x86_64; rv:101.0) Gecko/20100101 Firefox/101.0'
}

response = requests.get(url, headers=headers)
soup = BeautifulSoup(response.content, 'html.parser')

with open('page.html', 'w') as page:
    page.write(str(soup))

page.close()

print(soup.prettify()) 
