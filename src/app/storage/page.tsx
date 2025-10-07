"use client"
/**
 * Storage route for displaying, entering, withdrawing and editing storage entries.
 *
 * Data from backend will be received as follows:
 *
 * ```json
 * {
 *     "storageId": 1,
 *     "productName": "Brocoli",
 *     "freezerName": "Garage",
 *     "drawerName": "Schuif 1",
 *     "weightGrams": 525.3,
 *     "expirationDate": "2024-08-01",
 *     "expiresInDays": 128,
 *     "inStorageSince": "2023-08-01",
 *     "outStorageSince: "2024-08-01",
 * }
 * ```
 */
import {useState} from 'react';
import Link from 'next/link';
import FilterButtons from "@components/filter-buttons";
import {FaPenClip, FaTrashCan} from "react-icons/fa6"
import {createColumnHelper, getCoreRowModel, useReactTable} from "@tanstack/react-table";

import Table from '@components/table'
import ActionButton from "@components/action-button"
import {dummyStorage} from "@/assets/dummy-data";

/* Made for UI checking of overflowing and scrolling. DELETE LATER */
const dummyStorageExtended = dummyStorage.concat(
  dummyStorage.reduce((arr: Api.StorageResponse[], val, _) => {
    if (val.storageId === 4) {
      for (let idx = 0; idx < 50; idx++) {
        arr.push({
          ...val,
          storageId: idx + 5,
        })

      }
    }
    return arr
  }, [])
)

const columnHelper = createColumnHelper<Api.StorageResponse>()
const columns = [
  columnHelper.accessor('storageId', {
    cell: info => <i>{info.getValue()}</i>,
    header: () => <span>ID</span>,
  }),
  columnHelper.accessor('productName', {
    cell: info => <i>{info.getValue()}</i>,
    header: () => <span>Product</span>
  }),
  columnHelper.accessor('weightGrams', {
    cell: info => <i>{info.getValue() + ' '} g</i>,
    header: () => <span>Weight</span>,
  }),
  columnHelper.accessor('freezerName', {
    cell: info => <i>{info.getValue()}</i>,
    header: () => <span>Freezer</span>,
  }),
  columnHelper.accessor('drawerName', {
    cell: info => <i>{info.getValue()}</i>,
    header: () => <span>Drawer</span>,
  }),
  columnHelper.accessor('expirationDate', {
    cell: info => <i>{info.getValue().toDateString()}</i>,
    header: () => <span>Expiration Date</span>,
  }),
  columnHelper.display({
    id: 'actions',
    cell: () => <span className="flex flex-row flex-nowrap gap-1"><FaPenClip/>{"\t"}<FaTrashCan/></span>
  })
]

const Storage = () => {
  const [data, setData] = useState<Api.StorageResponse[]>(() => [...dummyStorageExtended])
  const [showModal, setShowModal] = useState<boolean>(false);
  const table = useReactTable({columns, data, getCoreRowModel: getCoreRowModel()})
  return (
    <div className={'content'}>
      <FilterButtons/>
      <Table columns={columns} data={data}/>
      <Link href={"/storage/withdraw"}><button>Withdraw an item</button></Link>
      <ActionButton
        id={"btn-storage-add"}
        action="add"
        href="/storage/add"
      />
    </div>
  )
}

export default Storage
