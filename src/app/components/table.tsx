import {ColumnDef, flexRender, getCoreRowModel, useReactTable} from "@tanstack/react-table";

type TableProps<T> = {
  columns: Array<ColumnDef<T, any>>,
  data: Array<T>,
  isExpandable?: boolean,
}

const Table = <T,>({columns, data}: TableProps<T>) => {
  const table = useReactTable({
    columns, data, getCoreRowModel: getCoreRowModel()
  })
  return (
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
  )
}

export default Table
