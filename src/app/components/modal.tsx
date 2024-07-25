import React from "react"
import ReactDOM from "react-dom"

import "@styles/globals.scss"
import style from "@styles/modals.module.scss"
import * as StorageModal from "@components/modal/storage"
import * as ProductModal from "@components/modal/product"
import * as FreezerModal from "@components/modal/freezer"
import {MouseEventHandler} from "react";
import {scryRenderedComponentsWithType} from "react-dom/test-utils";

export enum ModalType {
  AddStorage,
  WithdrawStorage,
  EditStorage,
  DeleteStorage,
  AddFreezer,
  EditFreezer,
  DeleteFreezer,
  AddProduct,
  EditProduct,
  DeleteProduct,
}

export type ModalProps = {
  kind: ModalType
  /* Expand the data type when necessary */
  data?: number
  children?: React.ReactNode
  onClose: () => void
}

const getModalContent = (kind: ModalType, data: ModalProps["data"], onClose: () => void): React.ReactNode => {
  switch (kind) {
    /* Storage */
    case ModalType.AddStorage:
      return StorageModal.AddStorage(onClose)
    case ModalType.DeleteStorage:
      return StorageModal.DeleteStorage(onClose)
    case ModalType.EditStorage:
      return StorageModal.EditStorage(onClose)
    case ModalType.WithdrawStorage:
      return StorageModal.WithdrawStorage(data, onClose)
    /* Products */
    case ModalType.AddProduct:
      return ProductModal.AddProduct(onClose)
    /* Freezers, including drawers */
    case ModalType.AddFreezer:
      return FreezerModal.AddFreezer(onClose)
    default:
      throw new Error(`Modal not configured for ${kind.toString()}`)
  }
}


const Modal = ({kind, data, onClose, children}: ModalProps) => {
  const modalInnerContent = getModalContent(kind, data, onClose)
  const modalRoot = document.getElementById("modal-root")
  if (!modalRoot) {
    throw new Error("HTML div with id 'modal-root' must be present in the document!")
  }

  const modalContent = (
    <div className={style.modalOverlay}>
      <div className={style.modal}>
        {modalInnerContent}
      </div>
    </div>
  )

  return ReactDOM.createPortal(
    modalContent,
    modalRoot
  )
}

export default Modal
