import requests

def main():

    pload = {'key':'value'}
    r = requests.post('http://localhost:9090/joinGame', data=pload)
    print(r.text)

main()
