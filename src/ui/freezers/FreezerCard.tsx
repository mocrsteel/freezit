import {CardContainer, CardContent, CardHeader} from "@/components/Card";

type CardProps = {
  key: string,
  freezer: DisplayFreezer
}

export function FreezerCard({key, freezer}: CardProps) {
  return (
    <CardContainer>
      <CardHeader title={freezer.name} editLink={`freezers/${freezer.freezerId}`}/>
      <CardContent>
        <div className="flex flex-col w-1/2 pl-4">
          <ul className="text-xs text-gray-500 flex flex-col gap-1">
            {freezer.drawers.length === 0
              ? <li>No drawers</li>
              : freezer.drawers.map((drawer, i) => (
                <li key={i}>{drawer.name}</li>
              ))
            }
          </ul>
        </div>
        <div className="flex flex-col w-1/2 items-center justify-center">
          <p className="text-gray-500 text-2xl font-extralight">{freezer.totalItemCount}</p>
          <p className="text-gray-500 text-xs">items in storage</p>
        </div>
      </CardContent>
    </CardContainer>
  )
}

