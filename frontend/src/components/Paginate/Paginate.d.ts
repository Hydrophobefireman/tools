import {Renderable} from "@hydrophobefireman/ui-lib";
interface PaginateProps<T> {
  atOnce: number;
  items: T[];
  render(item: T): Renderable;
  containerClass?: string | string[];
  nextButtonClass?: string | string[];
  previousButtonClass?: string | string[];
  buttonWrapperClass?: string;
  dualButtons?: boolean;
  nextText?: string;
  previousText?: string;
  buttonClass?: string;
}
export declare function Paginate<T>({
  atOnce,
  items,
  containerClass,
  render,
  nextButtonClass,
  previousButtonClass,
  buttonWrapperClass,
  dualButtons,
  nextText,
  previousText,
  buttonClass,
}: PaginateProps<T>): JSX.Element;
export {};
