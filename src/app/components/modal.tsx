import React from "react"
import ReactDOM from "react-dom"

import style from "@styles/modals.module.scss"
import * as StorageModal from "@components/modal/storage"
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

interface ModalContentProps {
  onClose: () => void
}
class ModalContent extends React.Component<ModalContentProps, {}> {

}

type ModalProps = {
  kind: ModalType
  children?: React.ReactNode
  onClose: () => void
}

const getModalContent = (kind: ModalType, onClose: () => void): React.ReactNode => {
  switch (kind) {
    case ModalType.AddStorage:
      return StorageModal.AddStorage(onClose)
    case ModalType.DeleteStorage:
      return StorageModal.DeleteStorage(onClose)
    case ModalType.EditStorage:
      return StorageModal.EditStorage(onClose)
    case ModalType.WithdrawStorage:
      return StorageModal.WithdrawStorage(onClose)
    default:
      throw new Error(`Modal not configured for ${kind.toString()}`)
  }
}


const Modal = ({kind, onClose, children}: ModalProps) => {
  const modalInnerContent = getModalContent(kind, onClose)
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
