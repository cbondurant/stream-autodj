use std::time::Duration;

use rodio::{source::Buffered, Sample, Source};

/// Internal function that builds a `RepeatCount` object.
pub fn repeat_with_count<I>(input: I, count: u32) -> RepeatCount<I>
where
	I: Source,
	I::Item: Sample,
{
	let input = input.buffered();
	RepeatCount {
		inner: input.clone(),
		next: input,
		count,
		count_remaining: count,
	}
}

/// A source that repeats the given source.
pub struct RepeatCount<I>
where
	I: Source,
	I::Item: Sample,
{
	inner: Buffered<I>,
	next: Buffered<I>,
	count: u32,
	count_remaining: u32,
}

impl<I> Iterator for RepeatCount<I>
where
	I: Source,
	I::Item: Sample,
{
	type Item = <I as Iterator>::Item;

	#[inline]
	fn next(&mut self) -> Option<Self::Item> {
		if let Some(value) = self.inner.next() {
			Some(value)
		}
		else if self.count_remaining > 1 {
			self.count_remaining -= 1;
			self.inner = self.next.clone();
			self.inner.next()
		}
		else {
			None
		}
	}

	#[inline]
	fn size_hint(&self) -> (usize, Option<usize>) {
		(0, None)
	}
}

impl<I> Source for RepeatCount<I>
where
	I: Iterator + Source,
	I::Item: Sample,
{
	#[inline]
	fn current_frame_len(&self) -> Option<usize> {
		match self.inner.current_frame_len() {
			Some(0) => self.next.current_frame_len(),
			a => a,
		}
	}

	#[inline]
	fn channels(&self) -> u16 {
		match self.inner.current_frame_len() {
			Some(0) => self.next.channels(),
			_ => self.inner.channels(),
		}
	}

	#[inline]
	fn sample_rate(&self) -> u32 {
		match self.inner.current_frame_len() {
			Some(0) => self.next.sample_rate(),
			_ => self.inner.sample_rate(),
		}
	}

	#[inline]
	fn total_duration(&self) -> Option<Duration> {
		match self.inner.total_duration() {
			Some(dur) => Some(dur.mul_f32(self.count as f32)),
			None => None,
		}
	}
}

impl<I> Clone for RepeatCount<I>
where
	I: Source,
	I::Item: Sample,
{
	#[inline]
	fn clone(&self) -> RepeatCount<I> {
		RepeatCount {
			inner: self.inner.clone(),
			next: self.next.clone(),
			count: self.count,
			count_remaining: self.count_remaining,
		}
	}
}
