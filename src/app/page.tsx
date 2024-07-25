"use client"
import {
  Chart,
  ChartData,
  ChartOptions,
  PluginChartOptions,
  CoreChartOptions,
  CategoryScale,
  LinearScale,
  BarController,
  BarElement,
  ChartEvent, ActiveElement
} from "chart.js"
import { Bar } from "react-chartjs-2"
import ChartDataLabels from "chartjs-plugin-datalabels"
import styleVars from "@styles/variables.module.scss"
import {useState, useEffect} from "react";
import ActionButton from "@components/action-button";
import { dummyStorageCount, dummyExpiresNext } from "@/app/assets/dummy-data";import Modal, { ModalType } from "./components/modal"
+

// Tree shaking to reduce amount of imported modules.
Chart.register(CategoryScale, LinearScale, BarController, BarElement, ChartDataLabels)

const AlmostEmptyCard = () => {
  const data: ChartData<"bar"> = {
    labels: dummyStorageCount.map(storage => storage.name),
    datasets: [{
      label: '',
      data: dummyStorageCount.map(storage => storage.count),
      backgroundColor: styleVars.ColorAccentDark,
      hoverBackgroundColor: styleVars.ColorShadeLight,
    }],
  }
  const plugins = [ChartDataLabels]
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
        }
      }
    },
    plugins: {
      datalabels: {
        color: "white",
        anchor: "end",
        align: "start",
      }
    }
  }
  return (
    <div id={'almost-empty-card'} className={'card-container'}>
      <h2 id={'almost-empty-title'}>Only a few left...</h2>
      <div className={'card'}>
        <div className={'graph'}>
          <Bar
            data={data}
            plugins={plugins}
            options={options}
          />
        </div>
      </div>
    </div>
  )
}

const UpNextCard = ({ item, onClick }: { item: Api.StorageResponse, onClick: () => void }) => {
  const [itemCount, setItemCount] = useState(3)

  return (
    <div id={'up-next-card'} className={'card-container'} onClick={onClick}>
      <h2 id={'up-next-title'}>What{"'"}s up next?</h2>
         <div className={'card'}>
          <div className={'info-box'}>
           <div id={'item-info'}>
             <h3>{item.productName}</h3>
             <p>Expires in {item.expiresInDays} days.</p>
             <p>Date: {item.expirationDate.toDateString()}</p>
             <p>{itemCount > 1
               ? `${itemCount} items left`
               : `${itemCount} item left`
             }</p>
           </div>
           <div id={'item-id'}>
             <p id={"item-id-number"}>{item.storageId}</p>
             <p id={"item-id-text"}>Storage ID</p>
           </div>
          </div>
         </div>
    </div>
  )
}

const Home = () => {
  const [showWithdrawModal, setShowWithdrawModal] = useState(false)
  const [itemId, setItemId] = useState<number>()
  
  useEffect(() => {
    const itemIdContainer = document.getElementById("item-id-number")
    if (itemIdContainer) {
      setItemId(Number(itemIdContainer.innerText))
    }
  }, [])
  return (
    <div className={"content"}>
      {showWithdrawModal &&
        <Modal kind={ModalType.WithdrawStorage} onClose={() => setShowWithdrawModal(false)} data={itemId}/>
      }
      <main>
        <h1>Welcome Back!</h1>
        <AlmostEmptyCard />
        <UpNextCard item={dummyExpiresNext} onClick={() => setShowWithdrawModal(true)} />
        <ActionButton id={"btn-withdraw-storage"} onClick={() => setShowWithdrawModal(true)} />
      </main>
    </div>
  )
}

export default Home
