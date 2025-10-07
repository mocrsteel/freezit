import {FaUpRightFromSquare, FaPlus} from "react-icons/fa6";
import Link from "next/link";

// import style from "@styles/action-button.module.scss"

type ActionHref = |
  "/storage/withdraw" |
  "/storage/create" |
  "/products/create" |
  "/freezers/create"

type ActionButtonProps = {
  id: string
  href: ActionHref
  action: "withdraw" | "add",
}

/**
 * Functional action button component.
 *
 * bottom padding is only pb-4 since parent has a pb-4 already.
 * @param props
 * @constructor
 */
export default function ActionButton({id, href, action}: ActionButtonProps) {
  return (
    <Link href={href}>
      <div className="sticky z-10 right-0 my-auto bottom-0 pt-8 pb-4 flex flex-col items-end pr-8">
        <button id={id}
                className="text-white w-fit text-xl p-4 bg-gradient-to-br rounded-full from-apple-500 to-apple-300 shadow-sm hover:from-apple-600 hover:to-apple-400 shadow-apple-400/50">
          {action === "withdraw"
            ? <FaUpRightFromSquare/>
            : <FaPlus/>
          }
        </button>
      </div>
    </Link>
  )
}
