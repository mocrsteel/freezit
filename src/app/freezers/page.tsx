"use client"

import {useReactTable, createColumnHelper, ColumnDef, getCoreRowModel, flexRender} from "@tanstack/react-table";
import { useState } from "react"
// import {Freezer} from "api-types";

const dummyData: Array<Api.Freezer> = [
  {
    freezerId: 1,
    name: "Garage"
  },
  {
    freezerId: 2,
    name: "Kelder"
  },
  {
    freezerId: 3,
    name: "Berging"
  },
  {
    freezerId: 4,
    name: "Gang"
  },
]
const columnHelper = createColumnHelper<Api.Freezer>()
const columns = [
  columnHelper.accessor('freezerId', {
    cell: info => <i>{info.getValue()}</i>,
    header: () => <span>Freezer ID</span>,
    footer: info => info.column.id
  }),
  columnHelper.accessor("name", {
    cell: info => <i>{info.getValue()}</i>,
    header: () => <span>Name</span>,
    footer: info => info.column.id
  })
]

const Freezers = () => {
  const [data, setDate] = useState<Api.Freezer[]>(() => [...dummyData])
  const table = useReactTable({ columns, data, getCoreRowModel: getCoreRowModel() })
  return (
    <div className={'content'}>
      <div className={'btn-container btn-dual'}>
        <button className={'btn btn-shade-light btn-dual'}>Filters</button>
        <button className={'btn btn-shade-light btn-dual'}>Remove Filters</button>
      </div>
      <div className={'table-container'}>
        <table>
          <thead>
          {table.getHeaderGroups().map(headerGroup => (
            <tr key={headerGroup.id}>
              {headerGroup.headers.map(header => (
                <th key={header.id}>
                  {header.isPlaceholder
                    ? null
                    : flexRender(header.column.columnDef.header, header.getContext())
                  }
                </th>
              ))}
            </tr>
          ))}
          </thead>
          <tbody>
          {table.getRowModel().rows.map(row => (
            <tr key={row.id}>
              {row.getVisibleCells().map(cell => (
                <td key={cell.id}>
                  {flexRender(cell.column.columnDef.cell, cell.getContext())}
                </td>
              ))}
            </tr>
          ))}
          </tbody>
        </table>
      </div>
    </div>
  )
}

export default Freezers
