import { faAnglesDown } from "@fortawesome/free-solid-svg-icons";
import {FontAwesomeIcon} from "@fortawesome/react-fontawesome";

import style from "@styles/action-button.module.scss"

type ArrowOrientation = "up" | "down"

type ActionButtonProps = {
  id: string
  onClick: () => void
  arrowUp?: boolean
}

/**
 * Functional action button component.
 * @param props
 * @constructor
 */
const ActionButton = ({id, onClick, arrowUp}: ActionButtonProps) => {
  return (
    <>
      <div className={style.actionButtonContainer}>
        <button id={id} className={style.actionButton + ' btn-primary'} onClick={onClick}>
          {arrowUp
            ? <FontAwesomeIcon icon={faAnglesDown} size={'xl'} rotation={180}/>
            : <FontAwesomeIcon icon={faAnglesDown} size={'xl'} />
          }
        </button>
        {arrowUp
          ? "Add"
          : "Withdraw"
        }
      </div>
    </>
  )
}

export default ActionButton
