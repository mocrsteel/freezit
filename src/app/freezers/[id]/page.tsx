import {dummyFreezerResponse} from "@/assets/dummy-data";
import {stringify} from "node:querystring";

export default async function FreezerDrawersPage({params}: {params: Promise<{id: string}>}) {
  const {id} = await params;
  const freezer = dummyFreezerResponse.filter(f => f.freezerId === Number(id));

  if (!freezer || (freezer && freezer.length < 0)) {
    return (
      <div>
        <h1>Error: Not found</h1>
        <p>No freezer was found with id {id}</p>
      </div>
    )
  }
  return (
    <div>
      <form>
        <p>Freezer: {freezer[0].name}</p>
        <p>ID: {freezer[0].freezerId}</p>
        <p>Items stored: {freezer[0].totalItemCount}</p>
        {freezer[0].drawers.map((d, index) => {
          return (
            <div key={d.drawerId} className="flex flex-row h-fit content-center items-center">
              <p className="font-bold w-10 text-center border border-red-500">{index + 1 }</p>
              <input type="text" value={d.name} className="border border-gray-300 px-3 py-2"/>
            </div>
          )
        })}
      </form>
    </div>
  )
}
