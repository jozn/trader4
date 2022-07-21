import * as fn from "./funcs";

(function() {
    setTimeout(run,50);
})();

function run() {
    console.log("------ Running run fn ----");
    makeBarChart();
    fn.checkboxChartChange();
    fn.buildOverlyHtml();
}

const ORANGE = 'rgb(224,91,0)';
var hide = {
    "trend_channel": true,
};

function makeBarChart() {
    var width =  window.innerWidth * 0.98;
    var chartMajorEl = document.getElementById("chart_major");
    var chartMediumEL = document.getElementById("chart_medium");
    var chartSmallEL = document.getElementById("chart_small");

    var jsonText = document.getElementById("json_data").innerText;
    var jd = JSON.parse(jsonText);

    // Create main Bar charts
    var chart_major = fn.buildBarChart({black: false, el: chartMajorEl, height: width/8, ohlc: jd.major.ohlc});
    var chart_medium = fn.buildBarChart({black: true, el: chartMediumEL, height: width/4, ohlc: jd.medium.ohlc,markers: jd.markers});
    var chart_small = fn.buildBarChart({black: false, el: chartSmallEL, height: width/6, ohlc: jd.small.ohlc,markers: jd.markers});
    // Syncs
    fn.syncCharts(chart_major,chart_medium);
    fn.syncCharts(chart_medium,chart_small);

    // chart_medium.remove()

    // Add bull/bear channel to main bars
    if(fn.getOverlyShow("trend_channel",true)) {
        fn.trendChannelChart(chart_major,jd.major);
        fn.trendChannelChart(chart_medium,jd.medium);
        fn.trendChannelChart(chart_small,jd.small);
    }

    // RPI indicaotr
    if(fn.getOverlyShow("rpi",true)) {
        // fn.rpiOverIndicator(chart_major,jd.major);
        fn.rpiOverIndicator(chart_medium, jd.medium);
        fn.rpiOverIndicator(chart_small, jd.medium);
    }

    // MA
    if(fn.getOverlyShow("ma1",true)) {
        fn.simpleLineOver(chart_major, jd.major.ma1);
        fn.simpleLineOver(chart_medium, jd.major.ma1);
        fn.simpleLineOver(chart_small, jd.major.ma1);
    }

    // DCSnake
    if(fn.getOverlyShow("dc_snake",false)) {
        fn.simpleLineOver(chart_medium,jd.medium.dcs_high);
        fn.simpleLineOver(chart_medium,jd.medium.dcs_low);
        fn.simpleLineOver(chart_medium,jd.medium.dcs_oversold,ORANGE,2);
    }

    ///////////////////////////  Dynamic Sub Charts ////////////////////////
    // Score
    var tscore_el = fn.makeNextIndi("tscore",true,true);
    var tscore_chart = fn.scoreChart(tscore_el,jd);
    fn.syncCharts(chart_medium,tscore_chart);

    // Trend Direction
    var tscore_el = fn.makeNextIndi("td_major",true,true);
    var tscore_chart = fn.trendDirectionChart(tscore_el,jd.major);
    fn.syncCharts(chart_medium,tscore_chart);
    var tscore_el = fn.makeNextIndi("td_medium",true,true);
    var tscore_chart = fn.trendDirectionChart(tscore_el,jd.medium);
    fn.syncCharts(chart_medium,tscore_chart);

    // MDI
    var medium_dmi_el = fn.makeNextIndi("medium_dmi",true,true);
    var medium_dmi = fn.mdi(medium_dmi_el,jd.medium);
    fn.syncCharts(chart_medium,medium_dmi);


    // Relative DC
    var rel_dc_el = fn.makeNextIndi("rel_dc_per",true,true);
    var medium_rdc = fn.relDc(rel_dc_el,jd);
    fn.syncCharts(chart_medium,medium_rdc);
    var rel_dc_el = fn.makeNextIndi("rel_dc_height",true,true);
    var medium_rdc = fn.relDcHeight(rel_dc_el,jd);
    fn.syncCharts(chart_medium,medium_rdc);


    // VelMom
    var vm1_el1 = fn.makeNextIndi("vm_mom",true,true);
    var vm1_chart1 = fn.onelineSubIndiacor(vm1_el1,jd.major.vm_mom);
    fn.syncCharts(chart_medium,vm1_chart1);

    var vm1_el1 = fn.makeNextIndi("vm_sum",true,true);
    var vm1_chart1 = fn.onelineSubIndiacor(vm1_el1,jd.major.vm_sum);
    fn.syncCharts(chart_medium,vm1_chart1);
    fn.syncCharts(chart_medium,vm1_chart1);

    var vm1_el1 = fn.makeNextIndi("vm_count",true,true);
    var vm1_chart1 = fn.onelineSubIndiacor(vm1_el1,jd.major.vm_count);
    fn.syncCharts(chart_medium,vm1_chart1);

    // Vel
    var vel1_el1 = fn.makeNextIndi("vel_avg_cnt",false,true);
    var vel1_chart1 = fn.onelineSubIndiacor(vel1_el1,jd.major.vel_avg);
    fn.syncCharts(chart_medium,vel1_chart1);

    var vel1_el1 = fn.makeNextIndi("vel_end",false,true);
    var vel1_chart1 = fn.onelineSubIndiacor(vel1_el1,jd.major.vel_end);
    fn.syncCharts(chart_medium,vel1_chart1);


    // new ma mom
    var ma_mom_el1 = fn.makeNextIndi("ma_mom_new",false,true);
    var ma_mom_chart1 = fn.onelineSubIndiacor(ma_mom_el1,jd.major.ma_mom);
    fn.syncCharts(chart_medium,ma_mom_chart1);

    // Sub Indicators
    let el_macd = fn.makeNextIndi("macd_major",true,true);
    // var macd_chart1 = fn.macdChart(el_macd,jd.medium);
    var macd_chart1 = fn.macdChart(el_macd,jd.major);
    fn.syncCharts(chart_medium,macd_chart1);

    // Score

    // MA Mom
    var ma_mom_el = fn.makeNextIndi("ma_mom",true,true);
    var ma_mom_chart = fn.maMomChart(ma_mom_el,jd);
    fn.syncCharts(chart_medium,ma_mom_chart);



    if(fn.getOverlyShow("zig_zag",false)) {
        var lowLine = chart_medium.addLineSeries({
            color: 'rgb(255,145,0)',
            lineWidth: 1.,
        });
        lowLine.setData(jd.zigzag);
    }
    
    // waves
    if(fn.getOverlyShow("waves",false)) {
        var wave1 = chart_medium.addLineSeries({
            color: 'rgb(13, 108, 56)', // 'rgb(0,255,0)',
            lineWidth: .8,
        });
        wave1.setData(jd.wave1);

        var wave2 = chart_medium.addLineSeries({
            color: 'rgb(255,0,0)',
            lineWidth: 0.6,
        });
        wave2.setData(jd.wave2);

        var wave3 = chart_medium.addLineSeries({
            color: 'rgb(0,0,255)',
            lineWidth: 1.,
        });
        wave3.setData(jd.wave3);
    }
}
