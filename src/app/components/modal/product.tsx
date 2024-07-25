import { useState, useEffect } from "react"

import style from "@styles/modals.module.scss"
import "@styles/globals.scss"

import { dummyProducts } from "@/app/assets/dummy-data"

const AddProduct = (onClose: () => void) => {
  const [productEntry, setProductEntry] = useState<Api.NewProduct>({name: "", expirationMonths: 1})
  const [productNameIsValid, setProductNameIsValid] = useState(true)
  const [expirationIsValid, setExpirationIsValid] = useState(true)
  const [showMonthsInput, setShowMonthsInput] = useState(false)
  
  const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    e.preventDefault()
    switch (e.currentTarget.id) {
      case "product-name":
        setProductEntry({
          ...productEntry,
          name: e.currentTarget.value
        })
        break
      case "product-expiration":
        if (productEntry) {
          setProductEntry({
            ...productEntry,
            expirationMonths: Number(e.currentTarget.value),
          })
        }
        break
    }
  }

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault()
  }
  
  useEffect(() => {
    const timeOutEntry = setTimeout(() => {
      const productsFiltered = dummyProducts.filter(product => product.name.toLowerCase() === productEntry.name.toLowerCase())
      if (productsFiltered.length > 0) {
        setProductNameIsValid(false)
        setShowMonthsInput(false)
      } else { 
        setProductNameIsValid(true)
        if (productEntry.name !== "") {
          setShowMonthsInput(true)
        } else {
          setShowMonthsInput(false)
        }
      }
      if (!productEntry.expirationMonths || productEntry.expirationMonths < 1) {
        setExpirationIsValid(false)
      } else {
        setExpirationIsValid(true)
      }
    }, 1000)
    return () => clearTimeout(timeOutEntry)
  }, [productEntry])

  return (
    <>
      <h1>Add a new product</h1>
      <form id="new-product-form" onSubmit={handleSubmit}>
        <label>Product name
          <input 
            id={"product-name"}
            className={!productNameIsValid ? style.invalidInput : ""}
            type={"text"} 
            onChange={handleChange}
            value={productEntry.name}
          />
        </label>
        {!productNameIsValid &&
          <p className={style.invalidInputMessage}>This product name already exists!</p>
        }
        {showMonthsInput &&
          // We only want to enter an expiration time when the product is defined to avoid an undefined name.
          <label>Expires after (months)
            <input id={"product-expiration"} className={!expirationIsValid ? style.invalidInput : ""} type={"number"} onChange={handleChange} value={Number(productEntry.expirationMonths)}/>
          </label>
        }
        {!expirationIsValid &&
          <p className={style.invalidInputMessage}>The expiration time should be at least 1 month,</p>
        }
      </form>
      <div className={style.modalButtons}>
        <button className={"btn btn-negative"} onClick={onClose}>Cancel</button>
        <button className={"btn btn-primary"}>Save</button>
      </div>
    </>
  )
}

const WithdrawProduct = () => {
  return (<></>)
}
const EditProduct = () => {
  return (<></>)
}
const DeleteProduct = () => {
  return (<></>)
}

export { AddProduct, WithdrawProduct, EditProduct, DeleteProduct }
