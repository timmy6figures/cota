import {Board, BoardProps} from "./components/Board"

const App = () => {
  const data: BoardProps = {cells: new Array(10).fill(new Array(9).fill({cell: null}))}
  return (
    <Board cells={data.cells}></Board>
  )
}

export default App