defmodule FreelancerRates do
  def daily_rate(hourly_rate) do
    hourly_rate * 8.0
  end

  def apply_discount(amount, discount) do
    amount * (1 - discount / 100)
  end

  def monthly_rate(hourly_rate, discount) do
    hourly_rate
    |> daily_rate()
    |> Kernel.*(22)
    |> apply_discount(discount)
    |> Float.ceil()
    |> trunc()
  end

  def days_in_budget(budget, hourly_rate, discount) do
    daily_cost =
      hourly_rate
      |> daily_rate()
      |> apply_discount(discount)

    budget
    |> Kernel./(daily_cost)
    |> Float.floor(1)
    
  end
end
