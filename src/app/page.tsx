"use client";
import {
  Chart,
  ChartData,
  ChartOptions,
  CategoryScale,
  LinearScale,
  BarController,
  BarElement,
} from "chart.js";
import {Bar} from "react-chartjs-2";
import ChartDataLabels from "chartjs-plugin-datalabels";
import React, {useState, useEffect} from "react";

import ActionButton from "@/components/action-button";
import {dummyStorageCount, dummyExpiresNext} from "@/assets/dummy-data";

// Tree shaking to reduce amount of imported modules.
Chart.register(
  CategoryScale,
  LinearScale,
  BarController,
  BarElement,
  ChartDataLabels,
);

interface CardProps extends React.HTMLAttributes<HTMLDivElement> {
  cardTitle?: string;
}

const Card: React.FC<CardProps> = (props) => {
  const {cardTitle, children, className, ...rest} = props;
  return (
    <div role="card-container" className={"my-1 " + className} {...rest}>
      {cardTitle ? <h2 className="font-normal text-lg mt-4 mb-2">{cardTitle}</h2> : null}
      <div
        role="card"
        className="rounded-lg shadow-md border-t border-l border-r border-slate-500/5 font-light text-sm"
      >
        {props.children}
      </div>
    </div>
  );
};

const AlmostEmptyCard = () => {
  const cardTitle = "Only a few left..."
  const data: ChartData<"bar"> = {
    labels: dummyStorageCount.map((storage) => storage.name),
    datasets: [
      {
        label: "",
        data: dummyStorageCount.map((storage) => storage.count),
        backgroundColor: "#26552c",
        hoverBackgroundColor: "#32873c",
      },
    ],
  };
  const plugins = [ChartDataLabels];
  const options: ChartOptions<"bar"> = {
    responsive: true,
    maintainAspectRatio: false,
    aspectRatio: 2,
    indexAxis: "y",
    layout: {
      padding: 16,
    },
    scales: {
      x: {
        display: false,
      },
      y: {
        border: {
          display: true,
          // color: styleVars.ColorBlack,
        },
        grid: {
          display: false,
        },
      },
    },
    plugins: {
      datalabels: {
        color: "white",
        anchor: "end",
        align: "start",
      },
    },
  };
  return (
    <Card id="almost-empty-card" cardTitle={cardTitle}>
      <div className="max-h-32 py-1">
        <Bar data={data} plugins={plugins} options={options}/>
      </div>
    </Card>
  );
};

interface UpNextCardProps extends React.HTMLProps<HTMLDivElement> {
  item: Api.StorageResponse[];
}

const UpNextCard: React.FC<UpNextCardProps> = (props) => {
  const cardTitle = "What's up next?";
  const [itemCount, setItemCount] = useState(3);
  const {item, ...rest} = props;

  const cards = item.map((item, idx) => {
    return (
      <Card key={idx} id={"up-next-card"} cardTitle={idx === 0 ? cardTitle : undefined} {...rest} className="p-0 gap-2">
        <div className="grid grid-cols-3">
          <div id="next-info" className="col-span-2 border-r border-r-slate-400/10 p-4">
            <h3 className="font-semibold">{item.productName}</h3>
            <p>Expires in {item.expiresInDays} days.</p>
            <p>Date: {item.expirationDate.toDateString()}</p>
            <p>
              {itemCount > 1
                ? `${itemCount} items left`
                : `${itemCount} item left`}
            </p>
          </div>
          <div id="item-id"
               className="h-full rounded-r-md col-span-1 flex flex-col place-content-center items-start pl-4 shadow-[inset_0_0_30px_-15px_rgba(0,0,0,0.6)]">
            <p id="item-id-number" className="text-3xl font-extralight">{item.storageId}</p>
            <p id="item-id-text" className="font-extralight">Storage ID</p>
          </div>
        </div>
      </Card>
    )
  })

  return <div className="flex flex-col gap-2">{cards}</div>
}

export default function HomePage() {
  const [itemId, setItemId] = useState<number>();

  useEffect(() => {
    const itemIdContainer = document.getElementById("item-id-number");
    if (itemIdContainer) {
      setItemId(Number(itemIdContainer.innerText));
    }
  }, []);

  const handleWithdrawClick = () => {
    window.alert("TODO: Implement open dialogs by address to make this button work properly!")
  }

  return (
    <div className="">
      <div>
        <h1 className="text-xl font-medium">Welcome Back!</h1>
        <AlmostEmptyCard/>
        <UpNextCard
          item={dummyExpiresNext}
          onClick={() => handleWithdrawClick()}
        />
      </div>
      <ActionButton id="withdraw-btn" href="/storage/withdraw" action="withdraw"/>
    </div>
  );
};
