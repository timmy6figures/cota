import React from "react";
import styles from "./Board.module.css"

const CellLayer: React.FC = () => {
    return (
        <div>
            <div className={styles["cell-layer-fragment"]}></div>
            <div className={styles["cell-layer-fragment"]}></div>
            <div className={styles["cell-layer-fragment"]}></div>
            <div className={styles["cell-layer-fragment"]}></div>
        </div>
    )
}

export default CellLayer