import style from "@styles/action-button.module.scss"
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

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault()
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
        <label>Freezer name
          <input id={"input-freezer-name"} type={"text"} />
        </label>
        {freezer.drawers.length > 0 &&
          freezer.drawers.map((name, idx) => {
            return (
              <DrawerInput idx={idx} name={name} updateDrawer={handleUpdate} onDelete={handleDelete} />
            )
          })
        }
        <button id="btn-add-drawer" className={"btn-negative " + style.centerButton} onClick={addDrawer}>Add drawer</button>
      </form>
      <div className={style.modalButtons}>
        <button id={"freezer-add-cancel"} className={"btn btn-negative"} onClick={onClose}>Cancel</button>
        <button id={"freezer-add-create"} className={"btn btn-primary"} onClick={onClose}>Create</button>
      </div>
      
    </>
  )
}

export const RemoveFreezer = () => { }

export const EditFreezer = () => { }