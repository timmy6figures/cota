import requests

def main():

    headers = {
            'User-Agent': 'Mozilla/5.0 (Macintosh; Intel Mac OS X 10.12; rv:55.0) Gecko/20100101 Firefox/55.0',
            }
    pload = {'key':'value'}
    r = requests.post('http://localhost:9090/createGame', data=pload)
    print(r.text)




main()
