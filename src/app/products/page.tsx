"use client"
import FilterButtons from "@components/filter-buttons";
import Table from "@components/table"
import ActionButton from "@components/action-button";
import {createColumnHelper} from "@tanstack/react-table";
import { dummyProducts } from "@/app/assets/dummy-data";

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
  return (
    <div className={'content'}>
      <FilterButtons />
      <Table columns={columns} data={dummyProducts} />
      <ActionButton id={"btn-product-add"} arrowUp/>
    </div>
  )
}

export default Products
