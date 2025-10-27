import {ColumnDef, flexRender, getCoreRowModel, useReactTable} from "@tanstack/react-table";

type TableProps<T> = {
  columns: Array<ColumnDef<T, any>>,
  data: Array<T>,
  isExpandable?: boolean,
}

export default function Table<T, >({columns, data}: TableProps<T>) {
  const table = useReactTable({
    columns, data, getCoreRowModel: getCoreRowModel()
  })
  return (
    <div className="flex flex-col w-full h-full items-center">
      <table className="table-fixed sm:table-auto overflow-x-scroll w-4/5 lg:w-3/4 max-w-[800px]">
        <thead className="relative">
        {table.getHeaderGroups().map(headerGroup => (
          <tr key={headerGroup.id} className="bg-apple-600 rounded-t text-white">
            {headerGroup.headers.map(header => (
              <th key={header.id} className="px-2 py-2">
                {header.isPlaceholder
                  ? null
                  : flexRender(header.column.columnDef.header, header.getContext())
                }
              </th>
            ))}
          </tr>
        ))}
        </thead>
        <tbody className="">
        {table.getRowModel().rows.map(row => (
          <tr key={row.id} className="border border-gray-400">
            {row.getVisibleCells().map(cell => (
              <td key={cell.id} className="px-2 py-2 border-y border-gray-400">
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
