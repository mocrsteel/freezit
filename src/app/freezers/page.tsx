"use client"

import {
  createColumnHelper,
  ExpandedState, flexRender,
  getCoreRowModel,
  getExpandedRowModel,
  useReactTable
} from "@tanstack/react-table"
import {FaCircleUp, FaCircleDown, FaCircleQuestion, FaPenClip, FaTrashCan} from "react-icons/fa6";

import React, {useState} from "react"
import {Tooltip} from "react-tooltip"

import FilterButtons from "@components/filter-buttons";
import ActionButton from "@components/action-button";
// import styleVars from "@styles/variables.module.scss"
import {dummyFreezerResponse} from "@/assets/dummy-data";

const columnHelper = createColumnHelper<DisplayFreezer>()

const freezerTooltipContent = (
  <div>
    If a row is not preceded by {<FaCircleDown/>}, no drawers have been specified for the given
    freezer! Consider adding at least one drawer before you can assign items in storage to it.
  </div>
)

const Freezers = () => {
  const [expanded, setExpanded] = useState<ExpandedState>({})
  const [data, setData] = useState<DisplayFreezer[]>(() => [...dummyFreezerResponse])

  const columns = [
    columnHelper.accessor("name", {
      header: () => <span>
        Name   <FaCircleQuestion
        id={"freezer-table-help"}
      />
      </span>,
      id: "freezer-name",
      cell: info => {
        return (
          <>
            {info.row.getCanExpand()
              ? (
                <span onClick={info.row.getToggleExpandedHandler()}>
                  {info.row.getIsExpanded()
                    ? <FaCircleUp/>
                    : <FaCircleDown/>
                  }
                </span>
              )
              : (<div id={"expand-placeholder"}></div>)
            }
            {"\t" + info.getValue()}
          </>
        )
      },
    }),
    columnHelper.accessor("totalItemCount", {
      header: "Items in storage",
      id: "storage-count",
      cell: info => info.getValue()
    }),
    columnHelper.accessor("freezerId", {
      id: 'actions',
      cell: info => <span>
        <FaPenClip/>
        {"\t"}
        <FaTrashCan/>
      </span>
    })
  ]

  const table = useReactTable({
    columns,
    data,
    state: {
      expanded
    },
    getExpandedRowModel: getExpandedRowModel(),
    getCoreRowModel: getCoreRowModel(),
    onExpandedChange: setExpanded,
    getSubRows: originalRow => (
      originalRow.drawers.map(drawer => ({
        freezerId: drawer.drawerId,
        name: drawer.name,
        totalItemCount: drawer.itemCount,
        drawers: [],
      }))
    )
  })
  return (
    <div className={'content'}>
      <Tooltip
        anchorSelect={"#freezer-table-help"}
        style={{maxWidth: "60vw", whiteSpace: "pre-wrap"}}
        opacity={0.85}
      >
        {freezerTooltipContent}
      </Tooltip>
      <FilterButtons/>
      <div className={'table-container'}>
        <table>
          <thead className={"shadow"}>
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
      <ActionButton id={"add-freezer-btn"} action="add" href="/freezers/create"/>
    </div>
  )
}

export default Freezers
