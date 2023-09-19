# Cota

## Front-End Design

### Datatypes

> Overview

```generalObj```

```
{
    dataType: null | string,
    data: Object
}
```

> In-ward Data Types

```gameStatusObj```
```
{
    dataType: "gameStatusObj"
    data:{
        playerId: string,
        turn: boolean,
        charge: integer,
        dead: object[],
        spare: object[],
        board: object[][]
    }
}
```

The ```gameStatus``` has to be sended to the players whenever either of them moved its piece.

```gameInfoObj```
```
{
    dataType: null | "gameInfoObj",
    data: {
        gameId: string,
        hostId: string,
        password: string
    }
}
```

```joinGameConfirmObj```
```
{
    dataType: null | "joinGameConfirmObj",
    data: {
        gameId: string,
        hostId: string,
        jointId: string,
        confirm: boolean
    }
}
```

> Out-going Data Types

```createGameObj```
```
{
    dataType: null | "createGameObj",
    data: {
        hostId: string,
        password: string
    }
}
```

```joinGameObj```
```
{
    dataType: null | "joinGameObj",
    data: {
        gameId: string,
        password: string,
        guestId: string
    }
}
```

```startGameObj```
```
{
    dataType: null | "startGameObj"
    data: {
        gameId: string,
        hostId: string,
        guestId: string,
        ready: boolean,
        startGame: boolean
    }
}
```

```exitGameObj```
```
{
    dataType: null | "exitGameObj",
    data: {
        gameId: string
    }
}
```

```playerMoveObj```
```
{
    dataType: null | "playerMoveObj",
    data: {
        gameId: string,
        playerId: string,
        piece: object,
        to: string
        // from: string,
    }
}
```

### Basic Logic Flow (Front-End)

> Creating a Game
1. *Host Player* requests ```createGameObj``` via ```Rest API```.
2. *Host Player* receives ```gameInfoObj``` via ```Rest API```.
3. *Host Player* will be redirected to the ```.../queue?id=<gameId>``` page.

- ```gameId``` and ```hostId``` have to be stored in encrypted ```cookie```.

> Joining a Game
1. *Guest Player* request ```joinGameObj``` via ```Rest API```.
2. *Guest Player* receives ```joinGameConfirmObj``` via ```Rest API```. 
3. Redirect to the ```.../queue?id=<gameId>``` page whenever the value of ```joinGameConfirmObj.data.confirm``` is ```true```.
4. *Host Player* receives ```joinGameConfirmObj``` via ```Web Socket```.
5. Update ```.../queue?id=<gameId>``` page UI showing both player.

> Starting a Game
1. *Host Player* request ```startGameObj``` via ```Rest API```.
2. Server sends ```startGameObj``` to both *Host Player* and *Guest Player*.
3. Whenever an error didn't occur, redirect to the page ```/game?gameId=<gameId>```.
4. Each *Player* receives ```gameStatusObj``` as *encrypted cookie* via ```Rest API``` to the *Host Player* and ```Web Socket``` to the *Guest Player*.

> While in a Game
1. Decrypt the ```gameStatusObj cookie```. 
2. Each *Player* who has ```<decrypted-gameStatusObj>.data.turn``` as ```true```, can play its turn.
3. Whenever the *Player* move its piece by drag-drop/clicking gridbox, the *Player* request ```playerMoveObj``` via ```Rest API```.
4. Both *Player* receives ```gameStatusObj``` as *encrypted cookie* via ```Rest API``` to the *Player* and ```Web Socket``` to the *Opponent*.
5. **Repeat!**

> Quit a Game
1. The *Player* requests ```exitGameObj``` via `Rest API` and automatically redirect to the ```.../score?gameId=<gameId>```.
2. The *Opponent* receives ```exitGameObj``` via `Web Socket` , and can redirect to the ```.../score?gameId=<gameId>```