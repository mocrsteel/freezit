"use client"
import {ChangeEvent, useState} from "react";
import {useRouter} from "next/navigation"

const tempFreezer = [
  {freezerId: 10, name: "Garage"}
]


export default function AddFreezer() {
  const [name, setName] = useState("")
  const router = useRouter()

  const handleChange = (e: ChangeEvent<HTMLInputElement>) => {
    e.preventDefault()
    if (e.currentTarget.id === "add-freezer-name") {
      setName(e.currentTarget.value)
    }
  }
  const handleSubmit = (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault()
    console.log(e.type)
    if (e.type === "submit") {
      if (tempFreezer.filter((freezer) => freezer.name === name).length === 0) {
        // todo: Get freezer from API by its name / return from the POST route.
        const newFreezerId = 10
        router.replace(`/freezers/create/success/${newFreezerId}`)
      } else {
        router.push(`/freezers/create/failure/${name}`)
      }
    }
  }

  return (
    <div className="w-full h-fit flex flex-col">
      <h1 className="my-4 w-full text-center">Create a new freezer</h1>
      <form id="add-freezer-form" className="mt-4 gap-6 flex flex-col" onSubmit={handleSubmit}>
        <input type="text" id="add-freezer-name" name="Name" placeholder="Freezer" value={name}
               onChange={(e) => setName(e.currentTarget.value)}
               className="border px-3 py-2 rounded-2xl focus:border-blue-300/30 border-gray-300"
               required/>
        <input type="submit" value="Create freezer"
               className="rounded-2xl px-3 py-2 text-white bg-apple-600 hover:bg-apple-600/90 hover:drop-shadow-sm transition-all hover:transition-all"/>
      </form>
    </div>
  )
}
