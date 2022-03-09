import * as LightweightCharts from 'lightweight-charts' ;
import {BarSeriesPartialOptions} from "lightweight-charts";

/// Playground ///
const zzchart = LightweightCharts.createChart(document.body, { width: 400, height: 300 });
const lineSeries = zzchart.addLineSeries();
lineSeries.setData([
    { time: '2019-04-11', value: 80.01 },
    { time: '2019-04-12', value: 96.63 },
    { time: '2019-04-13', value: 76.64 },
    { time: '2019-04-14', value: 81.89 },
    { time: '2019-04-15', value: 74.43 },
    { time: '2019-04-16', value: 80.01 },
    { time: '2019-04-17', value: 96.63 },
    { time: '2019-04-18', value: 76.64 },
    { time: '2019-04-19', value: 81.89 },
    { time: '2019-04-20', value: 74.43 },
]);

//////////////////
(function() {
    run();
})();

function run() {
    console.log("------ hi there ----");
    makeBarChart();
    var c =  $$("sdf");
}

function $$(id) {
    return document.getElementById(id);
}


// Make Bar Chart

function getChartCfg(width,height) {
    return {
        width: width,
        height: height,
        layout: {
            backgroundColor: '#ffffff',
            textColor: 'rgba(33, 56, 77, 1)',
        },
        crosshair: {
            mode: LightweightCharts.CrosshairMode.Normal,
        },
        rightPriceScale: {
            borderColor: 'rgba(197, 203, 206, 1)',
        },
        timeScale: {
            borderColor: 'rgba(197, 203, 206, 1)',
            secondsVisible: true,
            timeVisible: true,
            tickMarkFormatter: (time, tickMarkType, locale) => {
                var d = new Date(time * 1000);
                let day = d.getUTCDate();
                let hour = d.getUTCHours();
                let min = d.getUTCMinutes();
                let sec = d.getUTCSeconds();
                var str =  "" + day  + "  " + hour+":"+min+":"+sec ;
                return str
            },
        },
    }
}




function makeBarChart() {
    var width =  window.innerWidth * 0.98;
    var chartMajorEl = document.getElementById("chart_major");
    var chartMediumEL = document.getElementById("chart_medium");
    var chartSmallEL = document.getElementById("chart_small");

    var major_check_btn = document.getElementById("major_check_btn");
    var medium_check_btn = document.getElementById("medium_check_btn");
    var small_check_btn = document.getElementById("small_check_btn");

    var chart2El = document.getElementById("chart2");
    var chart3El = document.getElementById("chart3");
    var chart4El = document.getElementById("chart4");

    var jsonText = document.getElementById("json_data").innerText;
    var jsonData = JSON.parse(jsonText);
    var jd = jsonData;

    // ///////////////////////// Major Chart ////////////////////////
    // var chartMajor = LightweightCharts.createChart(chartMajorEl, getChartCfg(width, width/8));
    // var majorBarSeries = chartMajor.addBarSeries({
    //     thinBars: false,
    //     // downColor: '#000',
    //     // upColor: '#000',
    //     priceFormat: {
    //         minMove: 0.00001,
    //         precision: 5,
    //     },
    // });
    // majorBarSeries.setData(jd.major.ohlc);

    //
    var chart_major = buildBarChart({color: true, el: chartMajorEl, height: width/8, ohlc: jd.major.ohlc});
    var chart_medium = buildBarChart({color: false, el: chartMediumEL, height: width/5, ohlc: jd.medium.ohlc});
    var chart_small = buildBarChart({color: true, el: chartSmallEL, height: width/6, ohlc: jd.small.ohlc});


}

function buildBarChart(p: {el: HTMLElement,height:number,color:boolean,ohlc:any}) {
    var width =  window.innerWidth * 0.98;
    var downColor = '#000';
    var upColor = '#000';
    if (p.color) {
        downColor = '#000';
        upColor = '#000';
    }
    ///////////////////////// Major Chart ////////////////////////
    var chartMajor = LightweightCharts.createChart(p.el, getChartCfg(width, p.height));
    let barSeriesOpt :BarSeriesPartialOptions = {
        thinBars: false,
        // downColor: undefined,
        // upColor: undefined,
        priceFormat: {
            minMove: 0.00001,
            precision: 5,
        },
    };

    if (p.color) {
        barSeriesOpt.downColor = '#000';
        barSeriesOpt.upColor = '#000';
    }

    var majorBarSeries = chartMajor.addBarSeries(barSeriesOpt);
    majorBarSeries.setData(p.ohlc);
    return majorBarSeries;
}