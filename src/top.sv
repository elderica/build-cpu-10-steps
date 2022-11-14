module top (
    input wire clk,
    input wire button_s1,
    input wire button_s2,
    output wire [5:0] led_output
);

    logic [5:0] led = 'd0;

    assign led_output = ~led;

    always_ff @ (posedge clk) begin
        led[0] <= ~button_s1;
        led[5] <= ~button_s2;
    end
endmodule

`default_nettype wire