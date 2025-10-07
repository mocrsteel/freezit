"use client"

import {FaXmark} from 'react-icons/fa6'
import {useRouter} from 'next/navigation'

export default function Modal({children}: { children: React.ReactNode }) {
  const router = useRouter()

  return (
    <div id="modal-container" className="absolute z-50 h-full w-full backdrop-blur-md bg-black/50">
      <div id="modal-dialog" className="absolute left-1/2 top-1/2 -translate-x-1/2 -translate-y-1/2 h-fit w-fit bg-white rounded-xl flex flex-col">
        <div id="modal-header" className="flex w-full justify-end p-4 text-xl">
          <FaXmark className="hover:rotate-90 hover:transition-transform transition-transform" onClick={() => {router.back()}}/>
        </div>
        <div id="modal-content" className="w-full px-4 pb-4">{children}</div>
      </div>
    </div>
  )
}
