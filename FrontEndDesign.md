# Cota

## Front-End Design

### Datatypes

> Overview

```generalObj```

```
{
    data_type: null | string,
    data: Object
}
```

> Inbound Data Types

```gameStatusObj```
```
{
    data_type: "gameStatusObj"
    data:{
        player_id: string,
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
    data_type: null | "gameInfoObj",
    data: {
        game_id: string,
        host_id: string,
        password: string
    }
}
```

```joinGameConfirmObj```
```
{
    data_type: null | "joinGameConfirmObj",
    data: {
        game_id: string,
        host_id: string,
        join_id: string,
        confirm: boolean
    }
}
```

> Outbound Data Types

```createGameObj```
```
{
    data_type: null | "createGameObj",
    data: {
        host_id: string,
        password: string
    }
}
```

```joinGameObj```
```
{
    data_type: null | "joinGameObj",
    data: {
        game_id: string,
        password: string,
        guest_id: string
    }
}
```

```startGameObj```
```
{
    data_type: null | "startGameObj"
    data: {
        game_id: string,
        host_id: string,
        guest_id: string,
        ready: boolean,
        start_game: boolean
    }
}
```

```exitGameObj```
```
{
    data_type: null | "exitGameObj",
    data: {
        game_id: string
    }
}
```

```playerMoveObj```
```
{
    data_type: null | "playerMoveObj",
    data: {
        game_id: string,
        player_id: string,
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
