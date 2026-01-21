defmodule Lasagna do
  def expected_minutes_in_oven(), do: 40

  def remaining_minutes_in_oven(min) do 
    expected_minutes_in_oven() - min
  end

  def preparation_time_in_minutes(layer), do: layer * 2

  def total_time_in_minutes(layer , min) do
    preparation_time_in_minutes(layer) + min
  end

  def alarm, do: "Ding!" 
end
