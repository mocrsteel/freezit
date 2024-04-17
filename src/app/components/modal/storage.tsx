import React, {useState} from "react";
import {dummyDrawers, dummyFreezerResponse, dummyProducts} from "@/app/assets/dummy-data";

type AddStorageState = {
  productId: number
  freezerId: number
  drawerId: number
  weight?: number
}

const AddStorage = (onClose: () => void) => {
  // Expand this function to an API call later on.
  // Make sure to check that at least one freezer, one product and one drawer are defined.
  // In a next stage: consider making this part interactive "No freezer defined. Create one now?"
  // and "No products defined. Create one now?".
  const formInit = {
    productId: dummyProducts[0].productId,
    freezerId: dummyFreezerResponse[0].freezerId,
    drawerId: dummyFreezerResponse[0].drawers[0].drawerId
  }
  const [state, setState] = useState<AddStorageState>(formInit)
  const handleChange = (e: React.ChangeEvent<HTMLSelectElement | HTMLInputElement>) => {
    console.log(state)
    switch (e.currentTarget.id) {
      case "select-product":
        setState({
          ...state,
          productId: Number(e.currentTarget.value)
        })
        break
      case "select-freezer":
        setState({
          ...state,
          freezerId: Number(e.currentTarget.value)
        })
        break
      case "select-drawer":
        setState({
          ...state,
          drawerId: Number(e.currentTarget.value)
        })
        break
      case "input-weight":
        setState({
          ...state,
          weight: Number(e.currentTarget.value)
        })
    }
  }

  const handleSave = (e: React.MouseEvent<HTMLButtonElement>) => {
    e.preventDefault()
    if (e.currentTarget.id === "storage-add-enter") {
      if (!state.weight) {
        window.alert("You forgot to enter a weight!")
      } else {
        console.log(state)
        // Close this modal and then switch to the modal where it shows the entered ID!
      }
    }
  }

  return (
    <>
      <h1>Add an item</h1>
      <form>
        <label>Select product
          <select id={"select-product"} value={state.productId} onChange={handleChange}>
            {dummyProducts.map(product => {
              return (
                <option key={product.productId} value={product.productId}>{product.name}</option>
              )
            })}
          </select>
        </label>
        <label>Select freezer
          <select id={"select-freezer"} value={state.freezerId} onChange={handleChange}>
            {dummyFreezerResponse.map(freezer => {
              return (
                <option key={freezer.freezerId} value={freezer.freezerId}>{freezer.name}</option>
              )
            })}
          </select>
        </label>
        <label>Select drawer
          <select id={"select-drawer"} value={state.drawerId} onChange={handleChange}>
            {!state.freezerId &&
              <option>Select freezer first</option>
            }
            {state.freezerId &&
              // Select first element of array as we only should get one match per freezer id.
              dummyFreezerResponse.filter(freezer => freezer.freezerId === state.freezerId)[0].drawers.map(drawer => {
                return (
                  <option key={drawer.drawerId} value={drawer.drawerId}>{drawer.name}</option>
                )
              })
            }
          </select>
        </label>
        <label>Weight (grams)
          <input id={"input-weight"} type={"number"} value={state.weight} onChange={handleChange}/>
        </label>

      </form>
      <button id={"storage-add-cancel"} onClick={onClose}>Cancel</button>
      <button id={"storage-add-enter"} onClick={handleSave}>Enter item</button>
    </>
  )
}

const WithdrawStorage = (onClose: () => void) => {
  return (
    <>
      <button onClick={onClose}>x</button>
    </>
  )
}
const EditStorage = (onClose: () => void) => {
  return (
    <>
      <button onClick={onClose}>x</button>
    </>
  )
}
const DeleteStorage = (onClose: () => void) => {
  return (
    <>
      <button onClick={onClose}>x</button>
    </>
  )
}

export {AddStorage, WithdrawStorage, EditStorage, DeleteStorage}
