"use client"

import { createColumnHelper } from "@tanstack/react-table";
import { useState } from "react"

import ActionButton from "@components/action-button";
import { dummyProducts } from "@/app/assets/dummy-data";
import FilterButtons from "@components/filter-buttons";
import Modal, { ModalType } from "@components/modal"
import Table from "@components/table"

const columnHelper = createColumnHelper<Api.Product>()
const columns = [
  columnHelper.accessor('productId', {
    cell: content => content.getValue(),
    header: () => <span>ID</span>,
  }),
  columnHelper.accessor('name', {
    cell: content => content.getValue(),
    header: () => <span>Name</span>,
  }),
  columnHelper.accessor('expirationMonths', {
    cell: content => content.getValue() + " months",
    header: () => <span>Expires after</span>
  })
]

const Products = () => {
  const [showModal, setShowModal] = useState(false)

  return (
    <div className={'content'}>
      {showModal && <Modal kind={ModalType.AddProduct} onClose={() => setShowModal(false)} />}
      <FilterButtons />
      <Table columns={columns} data={dummyProducts} />
      <ActionButton id={"btn-product-add"} onClick={() => (setShowModal(true))} arrowUp />
    </div>
  )
}

export default Products
