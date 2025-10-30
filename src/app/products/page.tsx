"use client"

import {createColumnHelper} from "@tanstack/react-table";
import {useState} from "react"
import {CardGrid} from "@/components/Card"
import ProductCard from "@/ui/products/ProductsCard"

import ActionButton from "@/components/action-button"
import {dummyProducts} from "@/assets/dummy-data";
// import FilterButtons from "@/components/filter-buttons";
// import Table from "@/components/table"
//
// const columnHelper = createColumnHelper<Api.Product>()
// const columns = [
//   columnHelper.accessor('productId', {
//     cell: content => content.getValue(),
//     header: () => <span>ID</span>,
//   }),
//   columnHelper.accessor('name', {
//     cell: content => content.getValue(),
//     header: () => <span>Name</span>,
//   }),
//   columnHelper.accessor('expirationMonths', {
//     cell: content => content.getValue() + " months",
//     header: () => <span>Expires after</span>
//   })
// ]

const Products = () => {
  const [showModal, setShowModal] = useState(false)

  return (
    <div className={'content'}>
      <CardGrid>
        {dummyProducts && dummyProducts.map((product, index) => {
          return <ProductCard key={"product-" + index.toString()} product={product} />
        }
        )}
      </CardGrid>
      {/*<FilterButtons/>*/}
      {/*<Table columns={columns} data={dummyProducts}/>*/}
      <ActionButton id={"btn-product-add"} action="add" href="/products/create"/>
    </div>
  )
}

export default Products
