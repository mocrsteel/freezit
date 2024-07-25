"use client"

import {
  createColumnHelper,
  ExpandedState, flexRender,
  getCoreRowModel,
  getExpandedRowModel,
  useReactTable
} from "@tanstack/react-table"
import { faCircleUp, faCircleDown, faCircleQuestion } from "@fortawesome/free-regular-svg-icons";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import React, { useState } from "react"
import { Tooltip } from "react-tooltip"

import FilterButtons from "@components/filter-buttons";
import ActionButton from "@components/action-button";
import styleVars from "@styles/variables.module.scss"
import { dummyFreezerResponse } from "@/app/assets/dummy-data";
import Modal, { ModalType } from "../components/modal";

const columnHelper = createColumnHelper<DisplayFreezer>()
const columns = [
  columnHelper.accessor("name", {
    header: () => <span>
      Name   <FontAwesomeIcon
        id={"freezer-table-help"}
        icon={faCircleQuestion}
        size={"lg"}
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
                  ? <FontAwesomeIcon icon={faCircleUp} size={"lg"} color={styleVars.ColorAccentDark} />
                  : <FontAwesomeIcon icon={faCircleDown} size={"lg"} color={styleVars.ColorAccentLight} />
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
  })
]
const freezerTooltipContent = (
  <div>
    If a row is not preceded by {<FontAwesomeIcon icon={faCircleDown} />}, no drawers have been specified for the given
    freezer! Consider adding at least one drawer before you can assign items in storage to it.
  </div>
)

const Freezers = () => {
  const [showAddModal, setShowAddModal] = useState(false)
  const [expanded, setExpanded] = useState<ExpandedState>({})
  const [data, setData] = useState<DisplayFreezer[]>(() => [...dummyFreezerResponse])
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
        drawers: []
      }))
    )
  })
  return (
    <div className={'content'}>
      {showAddModal &&
        <Modal kind={ModalType.AddFreezer} onClose={() => setShowAddModal(false)} />
      }
      <Tooltip
        anchorSelect={"#freezer-table-help"}
        style={{ maxWidth: "60vw", whiteSpace: "pre-wrap" }}
        opacity={0.85}
      >
        {freezerTooltipContent}
      </Tooltip>
      <FilterButtons />
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
      <ActionButton id={"add-freezer-btn"} onClick={() => setShowAddModal(true)} arrowUp />
    </div>
  )
}

export default Freezers
