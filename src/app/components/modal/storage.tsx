import React, { useState, useEffect } from "react";
import { dummyDrawers, dummyFreezerResponse, dummyProducts, dummyStorage } from "@/app/assets/dummy-data";

import {ModalProps} from "@components/modal"
import style from "@styles/modals.module.scss"
import "@styles/globals.scss"

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
      } else if (!state.drawerId || state.drawerId === 0) {
        window.alert("No drawer was entered for this freezer.\nPlease creaet one first.")
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
          <input id={"input-weight"} type={"number"} value={state.weight} onChange={handleChange} />
        </label>
      </form>
      <div className={style.modalButtons}>
        <button id={"storage-add-cancel"} className={"btn btn-negative"} onClick={onClose}>Cancel</button>
        <button id={"storage-add-enter"} className={"btn btn-primary"} onClick={handleSave}>Enter item</button>
      </div>
    </>
  )
}

const WithdrawStorage = (data: ModalProps["data"], onClose: () => void) => {
  const [idState, setIdState] = useState<number | undefined>()
  const [state, setState] = useState<Api.StorageResponse | undefined>()
  const [idIsValid, setIdIsValid] = useState(true)
  
  // Run initialization to
  useEffect(() => {
    if (data && typeof data === "number") {
      setIdState(data)
    }
  }, [])
  useEffect(() => {
    const timeOutId = setTimeout(() => {
      console.warn("To implement ID GET request to backend on a change.")
      if (idState) { setState(getStorageItem(idState)) }
      else { 
        setState(undefined)
        setIdIsValid(true)
      }
    }, 500)
    return () => clearTimeout(timeOutId)
  })
  const handleIdChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    e.preventDefault()
    if (e.currentTarget.id === "input-product-id") {
      setIdState(Number(e.currentTarget.value))
    }
    /*  Idea: 
        on a change look up the value with a 0.5 second delay. Based on 200 or error response, 
        make the box red and give an error message "This storage ID does not exist."

        On status 200, show storage item details below in a nice layout and enable withdraw button.

    */
  }
  const getStorageItem = (id: number | undefined): Api.StorageResponse | undefined => {
    if (id) {
      const storageItems = dummyStorage.filter(stor => stor.storageId === id)
      if (storageItems.length > 0) {
        setIdIsValid(true)
        return storageItems[0]
      } else {
        setIdIsValid(false)
      }
    } else {
      setIdIsValid(false)
    }
  }

  const handleSave = () => {
    console.log(state)
    onClose()
  }

  return (
    <>
      <h1>Withdraw an item</h1>
      <form>
        <label>Storage ID
          <input
            id={"input-product-id"}
            type={"number"}
            value={idState}
            onChange={handleIdChange}
            className={idIsValid ? '' : style.invalidInput}
          />
        </label>
        {!idIsValid && <p className={style.invalidInputMessage}>This id does not exist in the database.</p>}
      </form>
      {state && idIsValid &&
        <div id={"storage-info"}>
          <div id="product-name-weight" className={style.productDescription}>
            <h2>{state.productName}</h2>
            <p>{state.weightGrams} g</p>
          </div>
          <ul style={{ textDecoration: "none" }}>
            <li><b>Freezer:{'\t'}</b>{state.freezerName}</li>
            <li><b>Drawer:{'\t'}</b>{state.drawerName}</li>
            <li><b>Expires:{'\t'}</b>{state.expirationDate.toDateString()}</li>
          </ul>
        </div>
      }
      <div className={style.modalButtons}>
        <button id={"storage-add-cancel"} className={"btn btn-negative"} onClick={onClose}>Cancel</button>
        <button id={"storage-add-enter"} className={"btn btn-primary"} onClick={handleSave}>Withdraw</button>
      </div>
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

export { AddStorage, WithdrawStorage, EditStorage, DeleteStorage }
