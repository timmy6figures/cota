import {Cell, CellProps} from './Cell'
import CellLayer from './CellLayer'
import styles from "./Board.module.css"

// 10 rows, 9 columns
export type BoardProps = {
    cells: Array<Array<CellProps>>
}

export const Board: React.FC<BoardProps> = (board) => {
    const flattened = board.cells.flatMap((row, i) => {
        return row.map((cell, j) => (<Cell cell={cell.cell} key={`${i}, ${j}`}></Cell>))
    })
    const layer = board.cells.flatMap((row, i) => {
        return row.map((cell, j) => (<CellLayer key={`${i}, ${j}`}></CellLayer>))
    })
    return (
        <div className={styles.board}>
            <div className={styles.realBoard}>
                {flattened}
            </div>
            <div className={styles.cellLayer}>
                {layer}
            </div>
        </div>
    )
}
