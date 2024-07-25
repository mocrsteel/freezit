import style from "@styles/modals.module.scss"
import { useState } from "react"

/**
 * All new drawers declared will be added to the freezer.
 * Handling of the freezerId for the drawers will be handled in the API call code.
 */
interface freezerState extends Api.NewFreezer {
  drawers: string[]
}

/**
 * Input props for unique drawer input lines.
 */
type DrawerInputProps = {
  idx: number,
  name: string,
  updateDrawer: (e: React.ChangeEvent<HTMLInputElement>) => void,
  onDelete: (e: React.MouseEvent<HTMLButtonElement>) => void,
}

/**
 * Type to define all possible input errors. Used to access the different error messages 
 * and to toggle their visibility individually.
 */
type FreezerInputErrors = {
  invalidFreezerName: {
    active: boolean,
    message: string,
  },
  invalidDrawerName: {
    // Opted for `boolean[]` to allow individually setting this message
    // per drawer input field.
    active: boolean[],
    message: string,
  },
  noDrawers: {
    active: boolean,
    message: string,
  },
  /*
  existingFreezerName: {
    active: false,
    message: string,
  }
  */
}

/**
 * 
 * @param index of drawer, drawer name, method to update the drawer, method to delete the drawer. 
 * @returns 
 */
const DrawerInput = ({ idx, name, updateDrawer, onDelete }: DrawerInputProps) => {
  return (
    <div key={"div-" + idx}>
      <label key={"lbl-" + idx}>Drawer {idx+1} name
        <div className={style.inputRow}>
          <input id={"input-drawer-" + idx.toString()} type="text" value={name} onChange={updateDrawer} />
          <button id={"delete-drawer-" + idx.toString()} className={"btn btn-negative"} onClick={onDelete}>Remove</button>
        </div>
      </label>
    </div>
  )
}

/**
 * Modal to allow an entry of a new freezer, including drawers.
 * @param onClose 
 * @returns JSX.Element
 */
export const AddFreezer = (onClose: () => void) => {
  const [freezer, updateFreezer] = useState<freezerState>({ name: "", drawers: [] })
  const [ inputErrors, setInputErrors ] = useState<FreezerInputErrors>({
    invalidFreezerName: {
      active: false,
      message: "Invalid freezer name. Name should be at least 3 characters long."
    },
    invalidDrawerName: {
      active: false,
      message: "Invalid drawer name. Name should be at least 3 characters long."
    },
    noDrawers: {
      active: false,
      message: "Add at least one drawer."
    }
  })

  /**
   * Helper function to check if any error is found in the inputs. Uses `inputError` state object.
   * @returns boolean
   */
  const checkInputisValid = (): boolean => {
    return Object.values(inputErrors).map(err => err.active).length === 0
  }

  /**
   * Checks the different input fields prior to submitting. Sets an error as active 
   * if an error is found and blocks the submit process.
   * @param e React.FormEvent
   */
  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault()
    if (e.currentTarget.id === "freezer-add-create") {
      
      // No drawers defined
      if (freezer.drawers.length === 0) {
        setInputErrors({
          ...inputErrors,
          noDrawers: {
            ...inputErrors.noDrawers,
            active: true,
          }
        })
      } else {
        setInputErrors({
          ...inputErrors,
          noDrawers: {
            ...inputErrors.noDrawers,
            active: false,
          }
        })
      }

      // Invalid freezer name
      if (freezer.name.length < 3) {
         setInputErrors({
          ...inputErrors,
          invalidFreezerName: {
            ...inputErrors.invalidFreezerName,
            active: true,
          }
         })
      } else {
        setInputErrors({
          ...inputErrors,
          invalidFreezerName: {
            ...inputErrors.invalidFreezerName,
            active: false,
          }
         })
      }
      
    }

    if (checkInputisValid()) {
      console.log(freezer)
    }
   }

  const handleUpdate = (e: React.ChangeEvent<HTMLInputElement>) => {
    e.preventDefault()
    const targetId = e.currentTarget.id.split("-")

    if (targetId[0] === "input") {
      switch (targetId[1]) {
        case "freezer":
          updateFreezer({
            ...freezer,
            name: e.currentTarget.value
          })
          break

        case "drawer":
          updateFreezer({
            ...freezer,
            drawers: freezer.drawers.map((name, idx) => {
              if (idx === Number(targetId.slice(-1)[0])) {
                return e.currentTarget.value
              } else {
                return name
              }
            })
          })
          break
      }
    }
  }

  const handleDelete = (e: React.MouseEvent<HTMLButtonElement>) => {
    e.preventDefault()
    const targetId = e.currentTarget.id.split("-")

    if (targetId[0] === "delete" && targetId[1] === "drawer") {
      console.log(targetId)
      updateFreezer({
        ...freezer,
        drawers: freezer.drawers.filter((name, idx) => {
          console.log(idx, targetId.slice(-1)[0])
          return idx !== Number(targetId.slice(-1)[0])
        })
      })
    }
  }

  const addDrawer = () => {
    updateFreezer({
      ...freezer,
      drawers: freezer.drawers.concat([""])
    })
  }
  return (
    <>
      <h1>Create new freezer</h1>
      <form onSubmit={handleSubmit}>
        <label className={style.mainLabel}>Freezer name
          <input id={"input-freezer-name"} className={style.mainInput} type={"text"} />
        </label>
        {inputErrors.noDrawers.active &&
          <p className={style.invalidInputMessage}>{inputErrors.noDrawers.message}</p>
        }
        {freezer.drawers.length > 0 &&
          freezer.drawers.map((name, idx) => {
            return (
              <DrawerInput idx={idx} name={name} updateDrawer={handleUpdate} onDelete={handleDelete} />
            )
          })
        }
        <div id="center-btn-container" className={style.centerButton}>
          <button id="btn-add-drawer" className={"btn btn-negative"} onClick={addDrawer}>Add drawer</button>
        </div>
      </form>
      <div className={style.modalButtons}>
        <button id={"freezer-add-cancel"} className={"btn btn-negative"} onClick={onClose}>Cancel</button>
        <button id={"freezer-add-create"} className={"btn btn-primary"} onClick={handleSubmit}>Create</button>
      </div>
      
    </>
  )
}

export const RemoveFreezer = () => { }

export const EditFreezer = () => { }