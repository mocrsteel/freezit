import { dummyFreezerResponse } from "@/assets/dummy-data";
import { FaPlus, FaSnowflake } from "react-icons/fa6";
import { BiArchive } from "react-icons/bi";
import { TextInput } from "@components/forms/input";

const FormDivider = () => {
  return <div className="h-[1px] bg-apple-800/40 mt-1 mb-6 " />;
};

export default function EditFreezer({ id }: { id: string }) {
  const freezers = dummyFreezerResponse.filter(
    (f) => f.freezerId === Number(id),
  );
  if (!freezers || (freezers && freezers.length < 1)) {
    return (
      <div>
        <h1>Error: Not found</h1>
        <p>No freezer was found with id {id}</p>
      </div>
    );
  }
  const freezer = freezers[0];
  const drawers = freezer.drawers;
  return (
    <div className="w-full flex flex-col items-center pt-4">
      <div className="flex flex-col w-2/3 max-w-[400px] items-center">
        <form className="text-gray-800 w-full">
          <p className="font-bold text-xl w-full">Freezer</p>
          <p className="text-xs text-gray-600 w-full">
            Database id: {freezer.freezerId}
          </p>
          <FormDivider />
          <TextInput
            inputId="freezer-id"
            subText={freezer.totalItemCount + " items in freezer"}
            placeholder={freezer.name}
            textValue={freezer.name}
            onChange={() => {}}
            onConfirm={() => {}}
            onDelete={() => {}}
            icon={<FaSnowflake />}
          />
          <p className="font-bold text-xl w-full mt-6">Drawers</p>
          <FormDivider />
          <div className="flex flex-col gap-6">
            {drawers.map((d, index) => {
              const subText =
                d.itemCount > 0 ? `Contains ${d.itemCount} items` : "Empty";
              return (
                <div key={d.drawerId + "-edit-group"} className="flex flex-col">
                  <TextInput
                    inputId={"drawer-" + d.drawerId.toString()}
                    textValue={""}
                    subText={subText}
                    onChange={() => null}
                    placeholder={d.name}
                    onConfirm={() => null}
                    onDelete={() => null}
                    icon={<BiArchive />}
                    deletable
                  />
                </div>
              );
            })}
          </div>
        </form>
        <button className="flex items-center gap-2 justify-center w-fit px-6 py-2 bg-apple-600 hover:bg-apple-600/90 transition-colors hover:transition-colors rounded-2xl text-white mt-12">
          <FaPlus className="text-lg" />
          Add drawer
        </button>
      </div>
    </div>
  );
}
