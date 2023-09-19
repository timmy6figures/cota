import {Cell, CellProps} from './Cell'
import { River } from './River'
import styles from "./Board.module.css"

// 10 rows, 9 columns
export type BoardProps = {
    cells: Array<Array<CellProps>>
}

export const Board: React.FC<BoardProps> = (board) => {
    const flattened = board.cells.flatMap((row, i) => {
        return row.map((cell, j) => (<Cell cell={cell.cell} key={`${i}, ${j}`}></Cell>))
    })
    const upper = flattened.slice(0, 45)
    const lower = flattened.slice(45)
    const rivers = Array(9).fill(null).map((_, j) => (<River key={`River ${j}`}></River>))
    const cells = upper.concat(rivers).concat(lower)
    return (
        <div className={styles.board}>
            {cells}
        </div>
    )
}
