import {TbDots} from 'react-icons/tb'
import Link from "next/link";

type CardHeaderProps = {
  title: string,
  editLink: string
}

export function CardHeader({title, editLink}: CardHeaderProps) {
  return (
    <div id="title-block"
         className="w-full relative flex flex-row justify-end align-middle text-center transition-all text-white p-4 font-normal text-xl bg-apple-600/70 group-hover:bg-apple-700/70 group-hover:transition-all rounded-lg shadow-md">
      <div className="w-full text-center">{title}</div>
      <Link href={editLink}><button className="absolute top-0 right-0 mr-4 h-full items-center"><TbDots/></button></Link>
    </div>
  )
}

export function CardContent({children}: { children: React.ReactNode }) {
  return <div className="flex h-full flex-row p-4 justify-center content-center items-center">{children}</div>
}

export function CardContainer({key, children}: { key: string, children: React.ReactNode }) {
  return (
    <div id="card-container"
         key={key}
         className="group max-w-96 w-full min-w-56 mx-auto text-gray-800 flex flex-col border border-t-0 rounded-lg border-green-700/20 focus:border-blue-300 shadow-md">
      {children}
    </div>
  )
}

export function CardGrid({children}: { children: React.ReactNode }) {
  return (
    <div id="grid-container" className="flex flex-col w-full items-center">
      <div id="card-grid" className="px-2 gap-4 w-10/12 grid grid-cols-1 md:grid-cols-2 md:gap-6 lg:grid-cols-3 lg:gap-8">
        {children}
      </div>
    </div>
  )
}
