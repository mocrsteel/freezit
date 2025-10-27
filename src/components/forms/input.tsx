import { FaRegCircleCheck, FaSnowflake } from "react-icons/fa6";
import { FaMinusCircle } from "react-icons/fa";
import { ChangeEvent } from "react";
import { IconType } from "react-icons";

type TextInputProps = {
  inputId: string;
  placeholder: string;
  textValue: string;
  subText?: string;
  icon?: React.ReactNode;
  deletable?: boolean;
  onChange: React.ChangeEventHandler<HTMLInputElement>;
  onConfirm: () => void;
  onDelete: () => void;
};

/**
 * Text input field. Styled to show the placeholder as label on the top border once in focus.
 * By setting prop `subText`, a smaller italic text will be shown at the bottom right of the
 * input field. Us this for additional contextual information when required or desired.
 * @param props
 * @constructor
 */
export function TextInput(props: TextInputProps) {
  console.warn("Event handlers have not been configured yet on `TextInput`");
  return (
    <div is="text-input-container">
      <div is="text-input-line" className="flex flex-row items-center gap-1">
        {props.icon && <div className="mr-4 text-2xl">{props.icon}</div>}
        <div className="relative w-full flex transition-all flex-row gap-2">
          <input
            type="text"
            id={"drawer-" + props.inputId}
            placeholder={props.placeholder}
            className="block px-2.5 pb-2.5 pt-4 w-full text-sm text-gray-900 bg-transparent rounded-lg border border-gray-300 appearance-none focus:outline-none focus:ring-0 focus:border-blue-600 peer"
          />
          <p className="absolute -translate-y-4 scale-75 top-2 text-sm transform transition-all duration-300 text-gray-500 px-2 z-10 origin-[0] bg-white peer-focus:text-blue-600 peer-placeholder-shown:scale-100 peer-placeholder-shown:-translate-y-1/2 peer-placeholder-shown:top-1/2 peer-focus:scale-75 peer-focus:top-2 peer-focus:-translate-y-4 rtl:peer-focus:left-auto start-1">
            {props.placeholder}
          </p>
          <button
            is="text-input-confirm-button"
            className="hidden peer-focus:block text-xl text-apple-500 hover:text-apple-600 transition-all"
          >
            <FaRegCircleCheck />
          </button>
          {props.deletable && (
            <button
              is="text-input-delete-button"
              className="peer-focus:block hidden text-xl text-red-600/90 hover:text-red-700/90"
            >
              <FaMinusCircle />
            </button>
          )}
        </div>
      </div>
      {props.subText && (
        <p className="w-full text-right text-xs px-2 pt-1 font-light italic text-gray-600">
          {props.subText}
        </p>
      )}
    </div>
  );
}
