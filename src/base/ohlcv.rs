// Licence: A modified version of yata library.
use super::candles::Source;

pub trait OHLCV {
    fn open(&self) -> f64;
    fn high(&self) -> f64;
    fn low(&self) -> f64;
    fn close(&self) -> f64;
    fn volume(&self) -> f64;

    // https://en.wikipedia.org/wiki/Typical_price
    #[inline]
    fn tp(&self) -> f64 {
        (self.high() + self.low() + self.close()) / 3.
    }

    #[inline]
    fn hlc3(&self) -> f64 {
        self.tp()
    }

    #[inline]
    fn hl2(&self) -> f64 {
        (self.high() + self.low()) * 0.5
    }

    fn ohlc4(&self) -> f64 {
        (self.high() + self.low() + self.close() + self.open()) * 0.25
    }

    // CLV = [(close - low) - (high - close)] / (high - low)
    #[inline]
    fn clv(&self) -> f64 {
        // we need to check division by zero, so we can really just check if `high` is equal to
        //  `low` without using any kind of round error checks.
        #[allow(clippy::float_cmp)]
        if self.high() == self.low() {
            0.
        } else {
            (2. * self.close() - self.low() - self.high()) / (self.high() - self.low())
        }
    }

    // https://en.wikipedia.org/wiki/Average_true_range over last two candles
    #[inline]
    fn tr(&self, prev_candle: &dyn OHLCV) -> f64 {
        self.tr_close(prev_candle.close())
    }

    // https://en.wikipedia.org/wiki/Average_true_range
    //  over last two candles using `close` price from the previous candle.
    #[inline]
    fn tr_close(&self, prev_close: f64) -> f64 {
        self.high().max(prev_close) - self.low().min(prev_close)
    }

    fn validate(&self) -> bool {
        !(self.close() > self.high() || self.close() < self.low() || self.high() < self.low())
            && self.close() > 0.
            && self.open() > 0.
            && self.high() > 0.
            && self.low() > 0.
            && self.close().is_finite()
            && self.open().is_finite()
            && self.high().is_finite()
            && self.low().is_finite()
            && (self.volume().is_nan() || self.volume() >= 0.0)
    }

    #[inline]
    fn source(&self, source: Source) -> f64 {
        match source {
            Source::Close => self.close(),
            Source::High => self.high(),
            Source::Low => self.low(),
            Source::TP => self.tp(),
            Source::HL2 => self.hl2(),
            Source::Volume => self.volume(),
            Source::VolumedPrice => self.volumed_price(),
            Source::Open => self.open(),
        }
    }

    fn volumed_price(&self) -> f64 {
        self.tp() * self.volume()
    }

    fn is_rising(&self) -> bool {
        self.close() > self.open()
    }

    fn is_falling(&self) -> bool {
        self.close() < self.open()
    }
}

impl OHLCV for (f64, f64, f64, f64, f64) {
    #[inline]
    fn open(&self) -> f64 {
        self.0
    }

    #[inline]
    fn high(&self) -> f64 {
        self.1
    }

    #[inline]
    fn low(&self) -> f64 {
        self.2
    }

    #[inline]
    fn close(&self) -> f64 {
        self.3
    }

    #[inline]
    fn volume(&self) -> f64 {
        self.4
    }
}

impl OHLCV for [f64; 5] {
    #[inline]
    fn open(&self) -> f64 {
        self[0]
    }

    #[inline]
    fn high(&self) -> f64 {
        self[1]
    }

    #[inline]
    fn low(&self) -> f64 {
        self[2]
    }

    #[inline]
    fn close(&self) -> f64 {
        self[3]
    }

    #[inline]
    fn volume(&self) -> f64 {
        self[4]
    }
}

impl<T: OHLCV> OHLCV for &T {
    #[inline]
    fn open(&self) -> f64 {
        (**self).open()
    }

    #[inline]
    fn high(&self) -> f64 {
        (**self).high()
    }

    #[inline]
    fn low(&self) -> f64 {
        (**self).low()
    }

    #[inline]
    fn close(&self) -> f64 {
        (**self).close()
    }

    #[inline]
    fn volume(&self) -> f64 {
        (**self).volume()
    }
}
